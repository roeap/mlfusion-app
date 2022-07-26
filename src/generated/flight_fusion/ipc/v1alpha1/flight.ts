/* eslint-disable */
import _m0 from "protobufjs/minimal";
import { DeltaOperationRequest } from "./delta";
import {
  CommandDropSource,
  CommandExecuteQuery,
  CommandKqlOperation,
  CommandReadDataset,
  CommandWriteIntoDataset,
  ResultActionStatus,
  ResultDoPutUpdate,
} from "./message";

/** Wrappers around to commands and actions tha get passed to the Flight service. */

/** Requests submitted against the `do_get` endpoint */
export interface FlightDoGetRequest {
  /** execute a KQL query against a registered Kusto cluster */
  kql:
    | CommandKqlOperation
    | undefined;
  /** Read data from a registered source */
  read:
    | CommandReadDataset
    | undefined;
  /** Execute a query against a pre-defined context */
  query:
    | CommandExecuteQuery
    | undefined;
  /** Perform a read operation against Delta table */
  delta: DeltaOperationRequest | undefined;
}

/** Requests submitted against the `do_put` endpoint */
export interface FlightDoPutRequest {
  /** Write data into a registered source */
  storage:
    | CommandWriteIntoDataset
    | undefined;
  /** Write data into delta table */
  delta: DeltaOperationRequest | undefined;
}

/** Response recieved from `do_put` operations` */
export interface FlightDoPutResponse {
  /** statistics for data written to source */
  update: ResultDoPutUpdate | undefined;
}

/** Requests submitted against the `do_action` endpoint */
export interface FlightActionRequest {
  /** command to remove a dataset from the area store */
  drop: CommandDropSource | undefined;
}

export interface FlightActionResponse {
  /** Result when actions reports its execution status */
  status: ResultActionStatus | undefined;
}

function createBaseFlightDoGetRequest(): FlightDoGetRequest {
  return { kql: undefined, read: undefined, query: undefined, delta: undefined };
}

export const FlightDoGetRequest = {
  encode(message: FlightDoGetRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.kql !== undefined) {
      CommandKqlOperation.encode(message.kql, writer.uint32(10).fork()).ldelim();
    }
    if (message.read !== undefined) {
      CommandReadDataset.encode(message.read, writer.uint32(18).fork()).ldelim();
    }
    if (message.query !== undefined) {
      CommandExecuteQuery.encode(message.query, writer.uint32(26).fork()).ldelim();
    }
    if (message.delta !== undefined) {
      DeltaOperationRequest.encode(message.delta, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FlightDoGetRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFlightDoGetRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.kql = CommandKqlOperation.decode(reader, reader.uint32());
          break;
        case 2:
          message.read = CommandReadDataset.decode(reader, reader.uint32());
          break;
        case 3:
          message.query = CommandExecuteQuery.decode(reader, reader.uint32());
          break;
        case 4:
          message.delta = DeltaOperationRequest.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FlightDoGetRequest {
    return {
      kql: isSet(object.kql) ? CommandKqlOperation.fromJSON(object.kql) : undefined,
      read: isSet(object.read) ? CommandReadDataset.fromJSON(object.read) : undefined,
      query: isSet(object.query) ? CommandExecuteQuery.fromJSON(object.query) : undefined,
      delta: isSet(object.delta) ? DeltaOperationRequest.fromJSON(object.delta) : undefined,
    };
  },

  toJSON(message: FlightDoGetRequest): unknown {
    const obj: any = {};
    message.kql !== undefined && (obj.kql = message.kql ? CommandKqlOperation.toJSON(message.kql) : undefined);
    message.read !== undefined && (obj.read = message.read ? CommandReadDataset.toJSON(message.read) : undefined);
    message.query !== undefined && (obj.query = message.query ? CommandExecuteQuery.toJSON(message.query) : undefined);
    message.delta !== undefined &&
      (obj.delta = message.delta ? DeltaOperationRequest.toJSON(message.delta) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<FlightDoGetRequest>, I>>(object: I): FlightDoGetRequest {
    const message = createBaseFlightDoGetRequest();
    message.kql = (object.kql !== undefined && object.kql !== null)
      ? CommandKqlOperation.fromPartial(object.kql)
      : undefined;
    message.read = (object.read !== undefined && object.read !== null)
      ? CommandReadDataset.fromPartial(object.read)
      : undefined;
    message.query = (object.query !== undefined && object.query !== null)
      ? CommandExecuteQuery.fromPartial(object.query)
      : undefined;
    message.delta = (object.delta !== undefined && object.delta !== null)
      ? DeltaOperationRequest.fromPartial(object.delta)
      : undefined;
    return message;
  },
};

function createBaseFlightDoPutRequest(): FlightDoPutRequest {
  return { storage: undefined, delta: undefined };
}

export const FlightDoPutRequest = {
  encode(message: FlightDoPutRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.storage !== undefined) {
      CommandWriteIntoDataset.encode(message.storage, writer.uint32(18).fork()).ldelim();
    }
    if (message.delta !== undefined) {
      DeltaOperationRequest.encode(message.delta, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FlightDoPutRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFlightDoPutRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 2:
          message.storage = CommandWriteIntoDataset.decode(reader, reader.uint32());
          break;
        case 3:
          message.delta = DeltaOperationRequest.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FlightDoPutRequest {
    return {
      storage: isSet(object.storage) ? CommandWriteIntoDataset.fromJSON(object.storage) : undefined,
      delta: isSet(object.delta) ? DeltaOperationRequest.fromJSON(object.delta) : undefined,
    };
  },

  toJSON(message: FlightDoPutRequest): unknown {
    const obj: any = {};
    message.storage !== undefined &&
      (obj.storage = message.storage ? CommandWriteIntoDataset.toJSON(message.storage) : undefined);
    message.delta !== undefined &&
      (obj.delta = message.delta ? DeltaOperationRequest.toJSON(message.delta) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<FlightDoPutRequest>, I>>(object: I): FlightDoPutRequest {
    const message = createBaseFlightDoPutRequest();
    message.storage = (object.storage !== undefined && object.storage !== null)
      ? CommandWriteIntoDataset.fromPartial(object.storage)
      : undefined;
    message.delta = (object.delta !== undefined && object.delta !== null)
      ? DeltaOperationRequest.fromPartial(object.delta)
      : undefined;
    return message;
  },
};

function createBaseFlightDoPutResponse(): FlightDoPutResponse {
  return { update: undefined };
}

export const FlightDoPutResponse = {
  encode(message: FlightDoPutResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.update !== undefined) {
      ResultDoPutUpdate.encode(message.update, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FlightDoPutResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFlightDoPutResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.update = ResultDoPutUpdate.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FlightDoPutResponse {
    return { update: isSet(object.update) ? ResultDoPutUpdate.fromJSON(object.update) : undefined };
  },

  toJSON(message: FlightDoPutResponse): unknown {
    const obj: any = {};
    message.update !== undefined &&
      (obj.update = message.update ? ResultDoPutUpdate.toJSON(message.update) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<FlightDoPutResponse>, I>>(object: I): FlightDoPutResponse {
    const message = createBaseFlightDoPutResponse();
    message.update = (object.update !== undefined && object.update !== null)
      ? ResultDoPutUpdate.fromPartial(object.update)
      : undefined;
    return message;
  },
};

function createBaseFlightActionRequest(): FlightActionRequest {
  return { drop: undefined };
}

export const FlightActionRequest = {
  encode(message: FlightActionRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.drop !== undefined) {
      CommandDropSource.encode(message.drop, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FlightActionRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFlightActionRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 2:
          message.drop = CommandDropSource.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FlightActionRequest {
    return { drop: isSet(object.drop) ? CommandDropSource.fromJSON(object.drop) : undefined };
  },

  toJSON(message: FlightActionRequest): unknown {
    const obj: any = {};
    message.drop !== undefined && (obj.drop = message.drop ? CommandDropSource.toJSON(message.drop) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<FlightActionRequest>, I>>(object: I): FlightActionRequest {
    const message = createBaseFlightActionRequest();
    message.drop = (object.drop !== undefined && object.drop !== null)
      ? CommandDropSource.fromPartial(object.drop)
      : undefined;
    return message;
  },
};

function createBaseFlightActionResponse(): FlightActionResponse {
  return { status: undefined };
}

export const FlightActionResponse = {
  encode(message: FlightActionResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.status !== undefined) {
      ResultActionStatus.encode(message.status, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FlightActionResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFlightActionResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.status = ResultActionStatus.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FlightActionResponse {
    return { status: isSet(object.status) ? ResultActionStatus.fromJSON(object.status) : undefined };
  },

  toJSON(message: FlightActionResponse): unknown {
    const obj: any = {};
    message.status !== undefined &&
      (obj.status = message.status ? ResultActionStatus.toJSON(message.status) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<FlightActionResponse>, I>>(object: I): FlightActionResponse {
    const message = createBaseFlightActionResponse();
    message.status = (object.status !== undefined && object.status !== null)
      ? ResultActionStatus.fromPartial(object.status)
      : undefined;
    return message;
  },
};

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

type DeepPartial<T> = T extends Builtin ? T
  : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
