import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh"

export interface InitializeFanoutArgsFields {
  epoch: number
}

export interface InitializeFanoutArgsJSON {
  epoch: number
}

export class InitializeFanoutArgs {
  readonly epoch: number

  constructor(fields: InitializeFanoutArgsFields) {
    this.epoch = fields.epoch
  }

  static layout(property?: string) {
    return borsh.struct([borsh.u32("epoch")], property)
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new InitializeFanoutArgs({
      epoch: obj.epoch,
    })
  }

  static toEncodable(fields: InitializeFanoutArgsFields) {
    return {
      epoch: fields.epoch,
    }
  }

  toJSON(): InitializeFanoutArgsJSON {
    return {
      epoch: this.epoch,
    }
  }

  static fromJSON(obj: InitializeFanoutArgsJSON): InitializeFanoutArgs {
    return new InitializeFanoutArgs({
      epoch: obj.epoch,
    })
  }

  toEncodable() {
    return InitializeFanoutArgs.toEncodable(this)
  }
}
