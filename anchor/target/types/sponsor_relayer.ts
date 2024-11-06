/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/sponsor_relayer.json`.
 */
export type SponsorRelayer = {
  "address": "ComputeBudget111111111111111111111111111111",
  "metadata": {
    "name": "sponsorRelayer",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [],
      "args": []
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidTipAccount",
      "msg": "Invalid tip account"
    },
    {
      "code": 6001,
      "name": "invalidMint",
      "msg": "Invalid mint"
    },
    {
      "code": 6002,
      "name": "invalidCloseAuthority",
      "msg": "Invalid close authority"
    },
    {
      "code": 6003,
      "name": "closeAuthorityNotSet",
      "msg": "Close authority not set"
    },
    {
      "code": 6004,
      "name": "invalidProgram",
      "msg": "Invalid program"
    },
    {
      "code": 6005,
      "name": "maxPriorityFeeExceeded",
      "msg": "Max priority fee exceeded"
    }
  ]
};
