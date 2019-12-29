import humanizeDuration from "humanize-duration";

export class MacroBlock {
  version: number;
  hash: string;
  epoch: number;
  pkey: string;
  difficulty: number;
  timestamp: string;
  blockReward: number;
  viewChange: number;
  gamma: string;
  validatorsLen: number;
  inputsLen: number;
  outputsLen: number;

  /// Constructor used to define order
  constructor() {
    this.epoch = 0;
    this.hash = "No block found";
    this.difficulty = 0;
    this.blockReward = 0;
    this.viewChange = 0;
    this.timestamp = "No block found";

    this.version = 0;
    this.pkey = "No block found";
    this.gamma = "No block found";
    this.inputsLen = 0;
    this.outputsLen = 0;
    this.validatorsLen = 0;
  }
}

export class MicroBlock {
  version: number;
  hash: string;
  epoch: number;
  offset: number;
  pkey: string;
  timestamp: string;
  transactionsLen: number;
  viewChange: number;
  inputsLen: number;
  outputsLen: number;
  /// Constructor used to define order
  constructor() {
    this.epoch = 0;
    this.offset = 0;
    this.hash = "No block found";
    this.transactionsLen = 0;
    this.timestamp = "No block found";
    this.viewChange = 0;

    this.version = 0;
    this.pkey = "No block found";
    this.inputsLen = 0;
    this.outputsLen = 0;
  }
}

export interface ValueFormat {
  text: string;
  copyable: boolean;
}

export class FieldProperty {
  name: string;
  truncate?: (title: string) => ValueFormat;

  constructor(name: string, truncate?: (title: string) => ValueFormat) {
    this.name = name;
    this.truncate = truncate;
  }
}
export function block_fields(): any {
  return {
    version: new FieldProperty("Version"),
    hash: new FieldProperty("Hash", format_hash),
    epoch: new FieldProperty("Epoch"),
    offset: new FieldProperty("Offset"),
    pkey: new FieldProperty("Leader", format_pkey),
    outputsLen: new FieldProperty("Outputs count"),
    inputsLen: new FieldProperty("Inputs count"),
    difficulty: new FieldProperty("Microblock difficulty"),
    timestamp: new FieldProperty("Timestamp"),
    blockReward: new FieldProperty("Block reward"),
    gamma: new FieldProperty("Gamma adjustment", format_gamma),
    validatorsLen: new FieldProperty("Count of validators"),
    transactionsLen: new FieldProperty("Count of transactions"),
    viewChange: new FieldProperty("Count of skiped leaders")
  };
}

export function get_duration(time: string): string {
  let date1 = Date.parse(time);
  let date2 = Date.now();
  return humanizeDuration(date2 - date1, { round: true });
}

export function format_hash(hash: string): ValueFormat {
  return {
    text: hash.slice(undefined, 32) + "...",
    copyable: true
  };
}

export function format_gamma(hash: string): ValueFormat {
  return {
    text: hash.slice(undefined, 32) + "...",
    copyable: false
  };
}

export function format_pkey(hash: string): ValueFormat {
  return {
    text: hash.slice(undefined, 32) + "...",
    copyable: true
  };
}

export function format_validator(hash: string): ValueFormat {
  return {
    text: hash.slice(undefined, 32) + "...",
    copyable: true
  };
}
