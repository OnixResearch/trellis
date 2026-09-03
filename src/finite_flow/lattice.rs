//! Finite join-table shape, law, and order admission.

fn reject(class: super::model::FailureClass, subject: Option<u32>) -> super::model::Rejection {
    super::model::Rejection {
        class,
        table: super::model::Table::Domain,
        index: None,
        subject,
    }
}

fn index(value: super::model::StateKey) -> Result<usize, super::model::Rejection> {
    usize::try_from(value.0).map_err(|_| reject(super::model::FailureClass::InvalidDomain, Some(value.0)))
}

fn table_index(
    domain: &super::model::Domain,
    left: super::model::StateKey,
    right: super::model::StateKey,
) -> Result<usize, super::model::Rejection> {
    if left.0 >= domain.state_count || right.0 >= domain.state_count {
        return Err(reject(super::model::FailureClass::InvalidDomain, Some(left.0.max(right.0))));
    }
    let row = left
        .0
        .checked_mul(domain.row_width)
        .and_then(|base| base.checked_add(right.0))
        .ok_or_else(|| reject(super::model::FailureClass::InvalidJoinTable, None))?;
    usize::try_from(row).map_err(|_| reject(super::model::FailureClass::InvalidJoinTable, Some(row)))
}

/// Returns one admitted finite join-table result.
// r[impl mir-dataflow.lattice-laws]
pub fn join_state(
    domain: &super::model::Domain,
    left: super::model::StateKey,
    right: super::model::StateKey,
) -> Result<super::model::StateKey, super::model::Rejection> {
    domain
        .join
        .get(table_index(domain, left, right)?)
        .copied()
        .ok_or_else(|| reject(super::model::FailureClass::InvalidJoinTable, None))
}

pub(super) fn state_is_ordered(
    domain: &super::model::Domain,
    left: super::model::StateKey,
    right: super::model::StateKey,
) -> Result<bool, super::model::Rejection> {
    Ok(join_state(domain, left, right)? == right)
}

fn check_shape(
    domain: &super::model::Domain,
) -> Result<(super::model::StateKey, super::model::StateKey, u32), super::model::Rejection> {
    if domain.state_count == 0 || domain.state_count > super::model::MAX_STATES {
        return Err(reject(super::model::FailureClass::InvalidDomain, Some(domain.state_count)));
    }
    if domain.row_width != domain.state_count {
        return Err(reject(super::model::FailureClass::InvalidJoinTable, Some(domain.row_width)));
    }
    let bottom = domain.bottom.ok_or_else(|| reject(super::model::FailureClass::MissingBottom, None))?;
    let top = domain.top.ok_or_else(|| reject(super::model::FailureClass::MissingTop, None))?;
    let height = domain.height.ok_or_else(|| reject(super::model::FailureClass::MissingHeight, None))?;
    if bottom.0 >= domain.state_count || top.0 >= domain.state_count {
        return Err(reject(super::model::FailureClass::InvalidDomain, Some(bottom.0.max(top.0))));
    }
    if height < domain.state_count || height > super::model::MAX_STATES {
        return Err(reject(super::model::FailureClass::InvalidDomain, Some(height)));
    }
    let expected = domain
        .state_count
        .checked_mul(domain.row_width)
        .ok_or_else(|| reject(super::model::FailureClass::InvalidJoinTable, None))?;
    let actual =
        u32::try_from(domain.join.len()).map_err(|_| reject(super::model::FailureClass::InvalidJoinTable, None))?;
    if actual != expected || domain.join.iter().any(|state| state.0 >= domain.state_count) {
        return Err(reject(super::model::FailureClass::InvalidJoinTable, Some(actual)));
    }
    Ok((bottom, top, height))
}

fn check_pair_laws(
    domain: &super::model::Domain,
    bottom: super::model::StateKey,
    top: super::model::StateKey,
) -> Result<(), super::model::Rejection> {
    for left_raw in 0..domain.state_count {
        let left = super::model::StateKey(left_raw);
        if join_state(domain, left, left)? != left
            || join_state(domain, bottom, left)? != left
            || join_state(domain, left, bottom)? != left
            || join_state(domain, top, left)? != top
            || join_state(domain, left, top)? != top
        {
            return Err(reject(super::model::FailureClass::LatticeLaw, Some(left_raw)));
        }
        for right_raw in 0..domain.state_count {
            let right = super::model::StateKey(right_raw);
            if join_state(domain, left, right)? != join_state(domain, right, left)? {
                return Err(reject(super::model::FailureClass::LatticeLaw, Some(left_raw)));
            }
        }
    }
    Ok(())
}

fn check_associativity(domain: &super::model::Domain) -> Result<(), super::model::Rejection> {
    for left_raw in 0..domain.state_count {
        let left = super::model::StateKey(left_raw);
        for middle_raw in 0..domain.state_count {
            let middle = super::model::StateKey(middle_raw);
            for right_raw in 0..domain.state_count {
                let right = super::model::StateKey(right_raw);
                let first = join_state(domain, join_state(domain, left, middle)?, right)?;
                let second = join_state(domain, left, join_state(domain, middle, right)?)?;
                if first != second {
                    return Err(reject(super::model::FailureClass::LatticeLaw, Some(left_raw)));
                }
            }
        }
    }
    Ok(())
}

/// Admits finite join shape and all semilattice laws exhaustively.
// r[impl mir-dataflow.domain]
// r[impl mir-dataflow.lattice-laws]
pub fn admit_domain(domain: &super::model::Domain) -> Result<(), super::model::Rejection> {
    let (bottom, top, _) = check_shape(domain)?;
    index(bottom)?;
    index(top)?;
    check_pair_laws(domain, bottom, top)?;
    check_associativity(domain)
}
