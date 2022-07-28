/* eslint-disable */
import {
  SaveMode,
  AreaSourceReference,
  SourceCollection,
  saveModeFromJSON,
  saveModeToJSON,
} from "./common";
import { SignalFrame } from "./signals";
import { BatchStatistics } from "./metadata";
import * as _m0 from "protobufjs/minimal";

export enum ActionStatus {
  ACTION_STATUS_UNSPECIFIED = 0,
  ACTION_STATUS_SUCCESS = 1,
  ACTION_STATUS_FAILURE = 2,
  UNRECOGNIZED = -1,
}

export function actionStatusFromJSON(object: any): ActionStatus {
  switch (object) {
    case 0:
    case "ACTION_STATUS_UNSPECIFIED":
      return ActionStatus.ACTION_STATUS_UNSPECIFIED;
    case 1:
    case "ACTION_STATUS_SUCCESS":
      return ActionStatus.ACTION_STATUS_SUCCESS;
    case 2:
    case "ACTION_STATUS_FAILURE":
      return ActionStatus.ACTION_STATUS_FAILURE;
    case -1:
    case "UNRECOGNIZED":
    default:
      return ActionStatus.UNRECOGNIZED;
  }
}

export function actionStatusToJSON(object: ActionStatus): string {
  switch (object) {
    case ActionStatus.ACTION_STATUS_UNSPECIFIED:
      return "ACTION_STATUS_UNSPECIFIED";
    case ActionStatus.ACTION_STATUS_SUCCESS:
      return "ACTION_STATUS_SUCCESS";
    case ActionStatus.ACTION_STATUS_FAILURE:
      return "ACTION_STATUS_FAILURE";
    case ActionStatus.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

/** Describes a KQL query operation */
export interface CommandKqlOperation {
  /** name of the Kusto service to be queried */
  serviceName: string;
  /** The KQL syntax. */
  query: string;
}

/** List all sources defined under an area node */
export interface CommandListSources {
  /** If true, all sources in child nodes are listed as well */
  recursive: boolean;
}

/** Read entire table from storage */
export interface CommandReadDataset {
  /** source identifier */
  source: AreaSourceReference | undefined;
  /** column selection to load */
  columnNames: string[];
}

/** Drop a source (e.g. a Table) from the service */
export interface CommandDropSource {
  /** source identifier */
  source: AreaSourceReference | undefined;
}

/** Request to write data to area storage */
export interface CommandWriteIntoDataset {
  /** source identifier */
  source: AreaSourceReference | undefined;
  /** denotes how to beahve for existing data - defaults to append */
  saveMode: SaveMode;
}

/** Execute a query against a given context */
export interface CommandExecuteQuery {
  query: string;
  source: AreaSourceReference | undefined;
  frame: SignalFrame | undefined;
  collection: SourceCollection | undefined;
}

/** result when a source is dropped */
export interface ResultActionStatus {
  status: ActionStatus;
}

export interface ResultDoPutUpdate {
  statistics: BatchStatistics | undefined;
}

function createBaseCommandKqlOperation(): CommandKqlOperation {
  return { serviceName: "", query: "" };
}

export const CommandKqlOperation = {
  encode(
    message: CommandKqlOperation,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.serviceName !== "") {
      writer.uint32(10).string(message.serviceName);
    }
    if (message.query !== "") {
      writer.uint32(18).string(message.query);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CommandKqlOperation {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCommandKqlOperation();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.serviceName = reader.string();
          break;
        case 2:
          message.query = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): CommandKqlOperation {
    return {
      serviceName: isSet(object.serviceName) ? String(object.serviceName) : "",
      query: isSet(object.query) ? String(object.query) : "",
    };
  },

  toJSON(message: CommandKqlOperation): unknown {
    const obj: any = {};
    message.serviceName !== undefined &&
      (obj.serviceName = message.serviceName);
    message.query !== undefined && (obj.query = message.query);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<CommandKqlOperation>, I>>(
    object: I
  ): CommandKqlOperation {
    const message = createBaseCommandKqlOperation();
    message.serviceName = object.serviceName ?? "";
    message.query = object.query ?? "";
    return message;
  },
};

function createBaseCommandListSources(): CommandListSources {
  return { recursive: false };
}

export const CommandListSources = {
  encode(
    message: CommandListSources,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.recursive === true) {
      writer.uint32(8).bool(message.recursive);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CommandListSources {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCommandListSources();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.recursive = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): CommandListSources {
    return {
      recursive: isSet(object.recursive) ? Boolean(object.recursive) : false,
    };
  },

  toJSON(message: CommandListSources): unknown {
    const obj: any = {};
    message.recursive !== undefined && (obj.recursive = message.recursive);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<CommandListSources>, I>>(
    object: I
  ): CommandListSources {
    const message = createBaseCommandListSources();
    message.recursive = object.recursive ?? false;
    return message;
  },
};

function createBaseCommandReadDataset(): CommandReadDataset {
  return { source: undefined, columnNames: [] };
}

export const CommandReadDataset = {
  encode(
    message: CommandReadDataset,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.source !== undefined) {
      AreaSourceReference.encode(
        message.source,
        writer.uint32(10).fork()
      ).ldelim();
    }
    for (const v of message.columnNames) {
      writer.uint32(18).string(v!);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CommandReadDataset {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCommandReadDataset();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.source = AreaSourceReference.decode(reader, reader.uint32());
          break;
        case 2:
          message.columnNames.push(reader.string());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): CommandReadDataset {
    return {
      source: isSet(object.source)
        ? AreaSourceReference.fromJSON(object.source)
        : undefined,
      columnNames: Array.isArray(object?.columnNames)
        ? object.columnNames.map((e: any) => String(e))
        : [],
    };
  },

  toJSON(message: CommandReadDataset): unknown {
    const obj: any = {};
    message.source !== undefined &&
      (obj.source = message.source
        ? AreaSourceReference.toJSON(message.source)
        : undefined);
    if (message.columnNames) {
      obj.columnNames = message.columnNames.map((e) => e);
    } else {
      obj.columnNames = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<CommandReadDataset>, I>>(
    object: I
  ): CommandReadDataset {
    const message = createBaseCommandReadDataset();
    message.source =
      object.source !== undefined && object.source !== null
        ? AreaSourceReference.fromPartial(object.source)
        : undefined;
    message.columnNames = object.columnNames?.map((e) => e) || [];
    return message;
  },
};

function createBaseCommandDropSource(): CommandDropSource {
  return { source: undefined };
}

export const CommandDropSource = {
  encode(
    message: CommandDropSource,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.source !== undefined) {
      AreaSourceReference.encode(
        message.source,
        writer.uint32(10).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CommandDropSource {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCommandDropSource();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.source = AreaSourceReference.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): CommandDropSource {
    return {
      source: isSet(object.source)
        ? AreaSourceReference.fromJSON(object.source)
        : undefined,
    };
  },

  toJSON(message: CommandDropSource): unknown {
    const obj: any = {};
    message.source !== undefined &&
      (obj.source = message.source
        ? AreaSourceReference.toJSON(message.source)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<CommandDropSource>, I>>(
    object: I
  ): CommandDropSource {
    const message = createBaseCommandDropSource();
    message.source =
      object.source !== undefined && object.source !== null
        ? AreaSourceReference.fromPartial(object.source)
        : undefined;
    return message;
  },
};

function createBaseCommandWriteIntoDataset(): CommandWriteIntoDataset {
  return { source: undefined, saveMode: 0 };
}

export const CommandWriteIntoDataset = {
  encode(
    message: CommandWriteIntoDataset,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.source !== undefined) {
      AreaSourceReference.encode(
        message.source,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.saveMode !== 0) {
      writer.uint32(24).int32(message.saveMode);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): CommandWriteIntoDataset {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCommandWriteIntoDataset();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.source = AreaSourceReference.decode(reader, reader.uint32());
          break;
        case 3:
          message.saveMode = reader.int32() as any;
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): CommandWriteIntoDataset {
    return {
      source: isSet(object.source)
        ? AreaSourceReference.fromJSON(object.source)
        : undefined,
      saveMode: isSet(object.saveMode) ? saveModeFromJSON(object.saveMode) : 0,
    };
  },

  toJSON(message: CommandWriteIntoDataset): unknown {
    const obj: any = {};
    message.source !== undefined &&
      (obj.source = message.source
        ? AreaSourceReference.toJSON(message.source)
        : undefined);
    message.saveMode !== undefined &&
      (obj.saveMode = saveModeToJSON(message.saveMode));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<CommandWriteIntoDataset>, I>>(
    object: I
  ): CommandWriteIntoDataset {
    const message = createBaseCommandWriteIntoDataset();
    message.source =
      object.source !== undefined && object.source !== null
        ? AreaSourceReference.fromPartial(object.source)
        : undefined;
    message.saveMode = object.saveMode ?? 0;
    return message;
  },
};

function createBaseCommandExecuteQuery(): CommandExecuteQuery {
  return {
    query: "",
    source: undefined,
    frame: undefined,
    collection: undefined,
  };
}

export const CommandExecuteQuery = {
  encode(
    message: CommandExecuteQuery,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.query !== "") {
      writer.uint32(10).string(message.query);
    }
    if (message.source !== undefined) {
      AreaSourceReference.encode(
        message.source,
        writer.uint32(82).fork()
      ).ldelim();
    }
    if (message.frame !== undefined) {
      SignalFrame.encode(message.frame, writer.uint32(90).fork()).ldelim();
    }
    if (message.collection !== undefined) {
      SourceCollection.encode(
        message.collection,
        writer.uint32(98).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CommandExecuteQuery {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCommandExecuteQuery();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.query = reader.string();
          break;
        case 10:
          message.source = AreaSourceReference.decode(reader, reader.uint32());
          break;
        case 11:
          message.frame = SignalFrame.decode(reader, reader.uint32());
          break;
        case 12:
          message.collection = SourceCollection.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): CommandExecuteQuery {
    return {
      query: isSet(object.query) ? String(object.query) : "",
      source: isSet(object.source)
        ? AreaSourceReference.fromJSON(object.source)
        : undefined,
      frame: isSet(object.frame)
        ? SignalFrame.fromJSON(object.frame)
        : undefined,
      collection: isSet(object.collection)
        ? SourceCollection.fromJSON(object.collection)
        : undefined,
    };
  },

  toJSON(message: CommandExecuteQuery): unknown {
    const obj: any = {};
    message.query !== undefined && (obj.query = message.query);
    message.source !== undefined &&
      (obj.source = message.source
        ? AreaSourceReference.toJSON(message.source)
        : undefined);
    message.frame !== undefined &&
      (obj.frame = message.frame
        ? SignalFrame.toJSON(message.frame)
        : undefined);
    message.collection !== undefined &&
      (obj.collection = message.collection
        ? SourceCollection.toJSON(message.collection)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<CommandExecuteQuery>, I>>(
    object: I
  ): CommandExecuteQuery {
    const message = createBaseCommandExecuteQuery();
    message.query = object.query ?? "";
    message.source =
      object.source !== undefined && object.source !== null
        ? AreaSourceReference.fromPartial(object.source)
        : undefined;
    message.frame =
      object.frame !== undefined && object.frame !== null
        ? SignalFrame.fromPartial(object.frame)
        : undefined;
    message.collection =
      object.collection !== undefined && object.collection !== null
        ? SourceCollection.fromPartial(object.collection)
        : undefined;
    return message;
  },
};

function createBaseResultActionStatus(): ResultActionStatus {
  return { status: 0 };
}

export const ResultActionStatus = {
  encode(
    message: ResultActionStatus,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.status !== 0) {
      writer.uint32(8).int32(message.status);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ResultActionStatus {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseResultActionStatus();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.status = reader.int32() as any;
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ResultActionStatus {
    return {
      status: isSet(object.status) ? actionStatusFromJSON(object.status) : 0,
    };
  },

  toJSON(message: ResultActionStatus): unknown {
    const obj: any = {};
    message.status !== undefined &&
      (obj.status = actionStatusToJSON(message.status));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ResultActionStatus>, I>>(
    object: I
  ): ResultActionStatus {
    const message = createBaseResultActionStatus();
    message.status = object.status ?? 0;
    return message;
  },
};

function createBaseResultDoPutUpdate(): ResultDoPutUpdate {
  return { statistics: undefined };
}

export const ResultDoPutUpdate = {
  encode(
    message: ResultDoPutUpdate,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.statistics !== undefined) {
      BatchStatistics.encode(
        message.statistics,
        writer.uint32(10).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ResultDoPutUpdate {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseResultDoPutUpdate();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.statistics = BatchStatistics.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ResultDoPutUpdate {
    return {
      statistics: isSet(object.statistics)
        ? BatchStatistics.fromJSON(object.statistics)
        : undefined,
    };
  },

  toJSON(message: ResultDoPutUpdate): unknown {
    const obj: any = {};
    message.statistics !== undefined &&
      (obj.statistics = message.statistics
        ? BatchStatistics.toJSON(message.statistics)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ResultDoPutUpdate>, I>>(
    object: I
  ): ResultDoPutUpdate {
    const message = createBaseResultDoPutUpdate();
    message.statistics =
      object.statistics !== undefined && object.statistics !== null
        ? BatchStatistics.fromPartial(object.statistics)
        : undefined;
    return message;
  },
};

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

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
