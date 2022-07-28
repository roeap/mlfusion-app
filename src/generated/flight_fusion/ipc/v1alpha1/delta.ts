/* eslint-disable */
import {
  SaveMode,
  AreaSourceReference,
  saveModeFromJSON,
  saveModeToJSON,
} from "./common";
import Long from "long";
import * as _m0 from "protobufjs/minimal";

export interface DeltaCreateOperation {
  saveMode: SaveMode;
  metadata: string;
}

export interface DeltaWriteOperation {
  saveMode: SaveMode;
  partitionBy: string[];
  predicate?: string | undefined;
}

export interface DeltaReadOperation {
  /** version of delta table to load */
  version: number;
  /** load delta version from point in time */
  timestamp: string;
  predicate: string;
  /** column selection to load */
  columnNames: string[];
}

export interface DeltaOperationRequest {
  source: AreaSourceReference | undefined;
  create: DeltaCreateOperation | undefined;
  write: DeltaWriteOperation | undefined;
  read: DeltaReadOperation | undefined;
}

export interface DeltaOperationResponse {
  stats: string;
}

function createBaseDeltaCreateOperation(): DeltaCreateOperation {
  return { saveMode: 0, metadata: "" };
}

export const DeltaCreateOperation = {
  encode(
    message: DeltaCreateOperation,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.saveMode !== 0) {
      writer.uint32(8).int32(message.saveMode);
    }
    if (message.metadata !== "") {
      writer.uint32(18).string(message.metadata);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): DeltaCreateOperation {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeltaCreateOperation();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.saveMode = reader.int32() as any;
          break;
        case 2:
          message.metadata = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DeltaCreateOperation {
    return {
      saveMode: isSet(object.saveMode) ? saveModeFromJSON(object.saveMode) : 0,
      metadata: isSet(object.metadata) ? String(object.metadata) : "",
    };
  },

  toJSON(message: DeltaCreateOperation): unknown {
    const obj: any = {};
    message.saveMode !== undefined &&
      (obj.saveMode = saveModeToJSON(message.saveMode));
    message.metadata !== undefined && (obj.metadata = message.metadata);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DeltaCreateOperation>, I>>(
    object: I
  ): DeltaCreateOperation {
    const message = createBaseDeltaCreateOperation();
    message.saveMode = object.saveMode ?? 0;
    message.metadata = object.metadata ?? "";
    return message;
  },
};

function createBaseDeltaWriteOperation(): DeltaWriteOperation {
  return { saveMode: 0, partitionBy: [], predicate: undefined };
}

export const DeltaWriteOperation = {
  encode(
    message: DeltaWriteOperation,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.saveMode !== 0) {
      writer.uint32(8).int32(message.saveMode);
    }
    for (const v of message.partitionBy) {
      writer.uint32(18).string(v!);
    }
    if (message.predicate !== undefined) {
      writer.uint32(26).string(message.predicate);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DeltaWriteOperation {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeltaWriteOperation();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.saveMode = reader.int32() as any;
          break;
        case 2:
          message.partitionBy.push(reader.string());
          break;
        case 3:
          message.predicate = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DeltaWriteOperation {
    return {
      saveMode: isSet(object.saveMode) ? saveModeFromJSON(object.saveMode) : 0,
      partitionBy: Array.isArray(object?.partitionBy)
        ? object.partitionBy.map((e: any) => String(e))
        : [],
      predicate: isSet(object.predicate) ? String(object.predicate) : undefined,
    };
  },

  toJSON(message: DeltaWriteOperation): unknown {
    const obj: any = {};
    message.saveMode !== undefined &&
      (obj.saveMode = saveModeToJSON(message.saveMode));
    if (message.partitionBy) {
      obj.partitionBy = message.partitionBy.map((e) => e);
    } else {
      obj.partitionBy = [];
    }
    message.predicate !== undefined && (obj.predicate = message.predicate);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DeltaWriteOperation>, I>>(
    object: I
  ): DeltaWriteOperation {
    const message = createBaseDeltaWriteOperation();
    message.saveMode = object.saveMode ?? 0;
    message.partitionBy = object.partitionBy?.map((e) => e) || [];
    message.predicate = object.predicate ?? undefined;
    return message;
  },
};

function createBaseDeltaReadOperation(): DeltaReadOperation {
  return { version: 0, timestamp: "", predicate: "", columnNames: [] };
}

export const DeltaReadOperation = {
  encode(
    message: DeltaReadOperation,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.version !== 0) {
      writer.uint32(8).int64(message.version);
    }
    if (message.timestamp !== "") {
      writer.uint32(18).string(message.timestamp);
    }
    if (message.predicate !== "") {
      writer.uint32(26).string(message.predicate);
    }
    for (const v of message.columnNames) {
      writer.uint32(34).string(v!);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DeltaReadOperation {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeltaReadOperation();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.version = longToNumber(reader.int64() as Long);
          break;
        case 2:
          message.timestamp = reader.string();
          break;
        case 3:
          message.predicate = reader.string();
          break;
        case 4:
          message.columnNames.push(reader.string());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DeltaReadOperation {
    return {
      version: isSet(object.version) ? Number(object.version) : 0,
      timestamp: isSet(object.timestamp) ? String(object.timestamp) : "",
      predicate: isSet(object.predicate) ? String(object.predicate) : "",
      columnNames: Array.isArray(object?.columnNames)
        ? object.columnNames.map((e: any) => String(e))
        : [],
    };
  },

  toJSON(message: DeltaReadOperation): unknown {
    const obj: any = {};
    message.version !== undefined &&
      (obj.version = Math.round(message.version));
    message.timestamp !== undefined && (obj.timestamp = message.timestamp);
    message.predicate !== undefined && (obj.predicate = message.predicate);
    if (message.columnNames) {
      obj.columnNames = message.columnNames.map((e) => e);
    } else {
      obj.columnNames = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DeltaReadOperation>, I>>(
    object: I
  ): DeltaReadOperation {
    const message = createBaseDeltaReadOperation();
    message.version = object.version ?? 0;
    message.timestamp = object.timestamp ?? "";
    message.predicate = object.predicate ?? "";
    message.columnNames = object.columnNames?.map((e) => e) || [];
    return message;
  },
};

function createBaseDeltaOperationRequest(): DeltaOperationRequest {
  return {
    source: undefined,
    create: undefined,
    write: undefined,
    read: undefined,
  };
}

export const DeltaOperationRequest = {
  encode(
    message: DeltaOperationRequest,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.source !== undefined) {
      AreaSourceReference.encode(
        message.source,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.create !== undefined) {
      DeltaCreateOperation.encode(
        message.create,
        writer.uint32(82).fork()
      ).ldelim();
    }
    if (message.write !== undefined) {
      DeltaWriteOperation.encode(
        message.write,
        writer.uint32(90).fork()
      ).ldelim();
    }
    if (message.read !== undefined) {
      DeltaReadOperation.encode(
        message.read,
        writer.uint32(98).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): DeltaOperationRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeltaOperationRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.source = AreaSourceReference.decode(reader, reader.uint32());
          break;
        case 10:
          message.create = DeltaCreateOperation.decode(reader, reader.uint32());
          break;
        case 11:
          message.write = DeltaWriteOperation.decode(reader, reader.uint32());
          break;
        case 12:
          message.read = DeltaReadOperation.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DeltaOperationRequest {
    return {
      source: isSet(object.source)
        ? AreaSourceReference.fromJSON(object.source)
        : undefined,
      create: isSet(object.create)
        ? DeltaCreateOperation.fromJSON(object.create)
        : undefined,
      write: isSet(object.write)
        ? DeltaWriteOperation.fromJSON(object.write)
        : undefined,
      read: isSet(object.read)
        ? DeltaReadOperation.fromJSON(object.read)
        : undefined,
    };
  },

  toJSON(message: DeltaOperationRequest): unknown {
    const obj: any = {};
    message.source !== undefined &&
      (obj.source = message.source
        ? AreaSourceReference.toJSON(message.source)
        : undefined);
    message.create !== undefined &&
      (obj.create = message.create
        ? DeltaCreateOperation.toJSON(message.create)
        : undefined);
    message.write !== undefined &&
      (obj.write = message.write
        ? DeltaWriteOperation.toJSON(message.write)
        : undefined);
    message.read !== undefined &&
      (obj.read = message.read
        ? DeltaReadOperation.toJSON(message.read)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DeltaOperationRequest>, I>>(
    object: I
  ): DeltaOperationRequest {
    const message = createBaseDeltaOperationRequest();
    message.source =
      object.source !== undefined && object.source !== null
        ? AreaSourceReference.fromPartial(object.source)
        : undefined;
    message.create =
      object.create !== undefined && object.create !== null
        ? DeltaCreateOperation.fromPartial(object.create)
        : undefined;
    message.write =
      object.write !== undefined && object.write !== null
        ? DeltaWriteOperation.fromPartial(object.write)
        : undefined;
    message.read =
      object.read !== undefined && object.read !== null
        ? DeltaReadOperation.fromPartial(object.read)
        : undefined;
    return message;
  },
};

function createBaseDeltaOperationResponse(): DeltaOperationResponse {
  return { stats: "" };
}

export const DeltaOperationResponse = {
  encode(
    message: DeltaOperationResponse,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.stats !== "") {
      writer.uint32(10).string(message.stats);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): DeltaOperationResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeltaOperationResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.stats = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DeltaOperationResponse {
    return {
      stats: isSet(object.stats) ? String(object.stats) : "",
    };
  },

  toJSON(message: DeltaOperationResponse): unknown {
    const obj: any = {};
    message.stats !== undefined && (obj.stats = message.stats);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DeltaOperationResponse>, I>>(
    object: I
  ): DeltaOperationResponse {
    const message = createBaseDeltaOperationResponse();
    message.stats = object.stats ?? "";
    return message;
  },
};

declare var self: any | undefined;
declare var window: any | undefined;
declare var global: any | undefined;
var globalThis: any = (() => {
  if (typeof globalThis !== "undefined") return globalThis;
  if (typeof self !== "undefined") return self;
  if (typeof window !== "undefined") return window;
  if (typeof global !== "undefined") return global;
  throw "Unable to locate global object";
})();

type Builtin =
  | Date
  | Function
  | Uint8Array
  | string
  | number
  | boolean
  | undefined;

type DeepPartial<T> = T extends Builtin
  ? T
  : T extends Array<infer U>
  ? Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U>
  ? ReadonlyArray<DeepPartial<U>>
  : T extends {}
  ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
type Exact<P, I extends P> = P extends Builtin
  ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & Record<
        Exclude<keyof I, KeysOfUnion<P>>,
        never
      >;

function longToNumber(long: Long): number {
  if (long.gt(Number.MAX_SAFE_INTEGER)) {
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  }
  return long.toNumber();
}

if (_m0.util.Long !== Long) {
  _m0.util.Long = Long as any;
  _m0.configure();
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
