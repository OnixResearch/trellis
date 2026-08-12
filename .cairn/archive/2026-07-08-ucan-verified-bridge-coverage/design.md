## Design

Define a coverage manifest whose rows name a UCAN bridge helper, Trellis primitive ID, source/proof digest, expected vendored path, evidence tier, and required non-claim boundary. A pure checker validates row shape and digest alignment against supplied Trellis and UCAN vendor metadata.

The manifest is not a UCAN correctness proof. It proves bridge-to-primitive identity and coverage only; UCAN still owns parser, token, resolver, replay, and shell orchestration boundaries.
