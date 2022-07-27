/* eslint-disable */
import Long from "long";
import { AreaSourceReference, Tag } from "./common";
import { Signal } from "./signals";
import * as _m0 from "protobufjs/minimal";

/** Metadata associated with an area source */
export interface AreaSourceMetadata {
  /** globally unique idetifier for the source */
  id: string;
  /** A human readable name for the source */
  name: string;
  /**
   * A short descrptive text that describes the content
   * and purpose of the data source
   */
  description: string;
  /** wether the table supports versioning */
  isVersioned: boolean;
  /** source identifier */
  source: AreaSourceReference | undefined;
  /** signals provided by area source */
  signals: Signal[];
  /** tags associated with source */
  tags: Tag[];
  /** user defined properties */
  properties: { [key: string]: string };
}

export interface AreaSourceMetadata_PropertiesEntry {
  key: string;
  value: string;
}

/** Detialed metadata and statistics about a source */
export interface AreaSourceDetails {
  /** globally unique idetifier for the source */
  id: string;
  /** Metadata associated with the source */
  metadata: AreaSourceMetadata | undefined;
}

/**
 * Statistics for a physical plan node
 * Fields are optional and can be inexact because the sources
 * sometimes provide approximate estimates for performance reasons
 * and the transformations output are not always predictable.
 */
export interface BatchStatistics {
  /** The number of table rows */
  recordCount: number;
  /** total byte of the table rows */
  totalByteSize: number;
  /** Statistics on a column level */
  columnStatistics: ColumnStatistics[];
  /**
   * If true, any field that is defined is the actual value in the data provided
   * by the operator (it is not an estimate). Any or all other fields might
   * still be None, in which case no information is known. if false, any field
   * that is has a value may contain an inexact estimate and may not be the
   * actual value.
   */
  isExact: boolean;
}

/** This table statistics are estimates about column properties */
export interface ColumnStatistics {
  /** Number of null values on column */
  nullCount: number;
  /** Maximum value of column */
  maxValue: string;
  /** Minimum value of column */
  minValue: string;
  /** Number of distinct values */
  distinctCount: number;
}

function createBaseAreaSourceMetadata(): AreaSourceMetadata {
  return {
    id: "",
    name: "",
    description: "",
    isVersioned: false,
    source: undefined,
    signals: [],
    tags: [],
    properties: {},
  };
}

export const AreaSourceMetadata = {
  encode(
    message: AreaSourceMetadata,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.id !== "") {
      writer.uint32(10).string(message.id);
    }
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    if (message.description !== "") {
      writer.uint32(26).string(message.description);
    }
    if (message.isVersioned === true) {
      writer.uint32(32).bool(message.isVersioned);
    }
    if (message.source !== undefined) {
      AreaSourceReference.encode(
        message.source,
        writer.uint32(42).fork()
      ).ldelim();
    }
    for (const v of message.signals) {
      Signal.encode(v!, writer.uint32(50).fork()).ldelim();
    }
    for (const v of message.tags) {
      Tag.encode(v!, writer.uint32(74).fork()).ldelim();
    }
    Object.entries(message.properties).forEach(([key, value]) => {
      AreaSourceMetadata_PropertiesEntry.encode(
        { key: key as any, value },
        writer.uint32(82).fork()
      ).ldelim();
    });
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AreaSourceMetadata {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAreaSourceMetadata();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = reader.string();
          break;
        case 2:
          message.name = reader.string();
          break;
        case 3:
          message.description = reader.string();
          break;
        case 4:
          message.isVersioned = reader.bool();
          break;
        case 5:
          message.source = AreaSourceReference.decode(reader, reader.uint32());
          break;
        case 6:
          message.signals.push(Signal.decode(reader, reader.uint32()));
          break;
        case 9:
          message.tags.push(Tag.decode(reader, reader.uint32()));
          break;
        case 10:
          const entry10 = AreaSourceMetadata_PropertiesEntry.decode(
            reader,
            reader.uint32()
          );
          if (entry10.value !== undefined) {
            message.properties[entry10.key] = entry10.value;
          }
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): AreaSourceMetadata {
    return {
      id: isSet(object.id) ? String(object.id) : "",
      name: isSet(object.name) ? String(object.name) : "",
      description: isSet(object.description) ? String(object.description) : "",
      isVersioned: isSet(object.isVersioned)
        ? Boolean(object.isVersioned)
        : false,
      source: isSet(object.source)
        ? AreaSourceReference.fromJSON(object.source)
        : undefined,
      signals: Array.isArray(object?.signals)
        ? object.signals.map((e: any) => Signal.fromJSON(e))
        : [],
      tags: Array.isArray(object?.tags)
        ? object.tags.map((e: any) => Tag.fromJSON(e))
        : [],
      properties: isObject(object.properties)
        ? Object.entries(object.properties).reduce<{ [key: string]: string }>(
            (acc, [key, value]) => {
              acc[key] = String(value);
              return acc;
            },
            {}
          )
        : {},
    };
  },

  toJSON(message: AreaSourceMetadata): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    message.name !== undefined && (obj.name = message.name);
    message.description !== undefined &&
      (obj.description = message.description);
    message.isVersioned !== undefined &&
      (obj.isVersioned = message.isVersioned);
    message.source !== undefined &&
      (obj.source = message.source
        ? AreaSourceReference.toJSON(message.source)
        : undefined);
    if (message.signals) {
      obj.signals = message.signals.map((e) =>
        e ? Signal.toJSON(e) : undefined
      );
    } else {
      obj.signals = [];
    }
    if (message.tags) {
      obj.tags = message.tags.map((e) => (e ? Tag.toJSON(e) : undefined));
    } else {
      obj.tags = [];
    }
    obj.properties = {};
    if (message.properties) {
      Object.entries(message.properties).forEach(([k, v]) => {
        obj.properties[k] = v;
      });
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<AreaSourceMetadata>, I>>(
    object: I
  ): AreaSourceMetadata {
    const message = createBaseAreaSourceMetadata();
    message.id = object.id ?? "";
    message.name = object.name ?? "";
    message.description = object.description ?? "";
    message.isVersioned = object.isVersioned ?? false;
    message.source =
      object.source !== undefined && object.source !== null
        ? AreaSourceReference.fromPartial(object.source)
        : undefined;
    message.signals = object.signals?.map((e) => Signal.fromPartial(e)) || [];
    message.tags = object.tags?.map((e) => Tag.fromPartial(e)) || [];
    message.properties = Object.entries(object.properties ?? {}).reduce<{
      [key: string]: string;
    }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = String(value);
      }
      return acc;
    }, {});
    return message;
  },
};

function createBaseAreaSourceMetadata_PropertiesEntry(): AreaSourceMetadata_PropertiesEntry {
  return { key: "", value: "" };
}

export const AreaSourceMetadata_PropertiesEntry = {
  encode(
    message: AreaSourceMetadata_PropertiesEntry,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== "") {
      writer.uint32(18).string(message.value);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): AreaSourceMetadata_PropertiesEntry {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAreaSourceMetadata_PropertiesEntry();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.key = reader.string();
          break;
        case 2:
          message.value = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): AreaSourceMetadata_PropertiesEntry {
    return {
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value) ? String(object.value) : "",
    };
  },

  toJSON(message: AreaSourceMetadata_PropertiesEntry): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined && (obj.value = message.value);
    return obj;
  },

  fromPartial<
    I extends Exact<DeepPartial<AreaSourceMetadata_PropertiesEntry>, I>
  >(object: I): AreaSourceMetadata_PropertiesEntry {
    const message = createBaseAreaSourceMetadata_PropertiesEntry();
    message.key = object.key ?? "";
    message.value = object.value ?? "";
    return message;
  },
};

function createBaseAreaSourceDetails(): AreaSourceDetails {
  return { id: "", metadata: undefined };
}

export const AreaSourceDetails = {
  encode(
    message: AreaSourceDetails,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.id !== "") {
      writer.uint32(10).string(message.id);
    }
    if (message.metadata !== undefined) {
      AreaSourceMetadata.encode(
        message.metadata,
        writer.uint32(18).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AreaSourceDetails {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAreaSourceDetails();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = reader.string();
          break;
        case 2:
          message.metadata = AreaSourceMetadata.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): AreaSourceDetails {
    return {
      id: isSet(object.id) ? String(object.id) : "",
      metadata: isSet(object.metadata)
        ? AreaSourceMetadata.fromJSON(object.metadata)
        : undefined,
    };
  },

  toJSON(message: AreaSourceDetails): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    message.metadata !== undefined &&
      (obj.metadata = message.metadata
        ? AreaSourceMetadata.toJSON(message.metadata)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<AreaSourceDetails>, I>>(
    object: I
  ): AreaSourceDetails {
    const message = createBaseAreaSourceDetails();
    message.id = object.id ?? "";
    message.metadata =
      object.metadata !== undefined && object.metadata !== null
        ? AreaSourceMetadata.fromPartial(object.metadata)
        : undefined;
    return message;
  },
};

function createBaseBatchStatistics(): BatchStatistics {
  return {
    recordCount: 0,
    totalByteSize: 0,
    columnStatistics: [],
    isExact: false,
  };
}

export const BatchStatistics = {
  encode(
    message: BatchStatistics,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.recordCount !== 0) {
      writer.uint32(8).int64(message.recordCount);
    }
    if (message.totalByteSize !== 0) {
      writer.uint32(16).int64(message.totalByteSize);
    }
    for (const v of message.columnStatistics) {
      ColumnStatistics.encode(v!, writer.uint32(26).fork()).ldelim();
    }
    if (message.isExact === true) {
      writer.uint32(32).bool(message.isExact);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): BatchStatistics {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseBatchStatistics();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.recordCount = longToNumber(reader.int64() as Long);
          break;
        case 2:
          message.totalByteSize = longToNumber(reader.int64() as Long);
          break;
        case 3:
          message.columnStatistics.push(
            ColumnStatistics.decode(reader, reader.uint32())
          );
          break;
        case 4:
          message.isExact = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): BatchStatistics {
    return {
      recordCount: isSet(object.recordCount) ? Number(object.recordCount) : 0,
      totalByteSize: isSet(object.totalByteSize)
        ? Number(object.totalByteSize)
        : 0,
      columnStatistics: Array.isArray(object?.columnStatistics)
        ? object.columnStatistics.map((e: any) => ColumnStatistics.fromJSON(e))
        : [],
      isExact: isSet(object.isExact) ? Boolean(object.isExact) : false,
    };
  },

  toJSON(message: BatchStatistics): unknown {
    const obj: any = {};
    message.recordCount !== undefined &&
      (obj.recordCount = Math.round(message.recordCount));
    message.totalByteSize !== undefined &&
      (obj.totalByteSize = Math.round(message.totalByteSize));
    if (message.columnStatistics) {
      obj.columnStatistics = message.columnStatistics.map((e) =>
        e ? ColumnStatistics.toJSON(e) : undefined
      );
    } else {
      obj.columnStatistics = [];
    }
    message.isExact !== undefined && (obj.isExact = message.isExact);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<BatchStatistics>, I>>(
    object: I
  ): BatchStatistics {
    const message = createBaseBatchStatistics();
    message.recordCount = object.recordCount ?? 0;
    message.totalByteSize = object.totalByteSize ?? 0;
    message.columnStatistics =
      object.columnStatistics?.map((e) => ColumnStatistics.fromPartial(e)) ||
      [];
    message.isExact = object.isExact ?? false;
    return message;
  },
};

function createBaseColumnStatistics(): ColumnStatistics {
  return { nullCount: 0, maxValue: "", minValue: "", distinctCount: 0 };
}

export const ColumnStatistics = {
  encode(
    message: ColumnStatistics,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.nullCount !== 0) {
      writer.uint32(8).int64(message.nullCount);
    }
    if (message.maxValue !== "") {
      writer.uint32(18).string(message.maxValue);
    }
    if (message.minValue !== "") {
      writer.uint32(26).string(message.minValue);
    }
    if (message.distinctCount !== 0) {
      writer.uint32(32).int64(message.distinctCount);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ColumnStatistics {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseColumnStatistics();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.nullCount = longToNumber(reader.int64() as Long);
          break;
        case 2:
          message.maxValue = reader.string();
          break;
        case 3:
          message.minValue = reader.string();
          break;
        case 4:
          message.distinctCount = longToNumber(reader.int64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ColumnStatistics {
    return {
      nullCount: isSet(object.nullCount) ? Number(object.nullCount) : 0,
      maxValue: isSet(object.maxValue) ? String(object.maxValue) : "",
      minValue: isSet(object.minValue) ? String(object.minValue) : "",
      distinctCount: isSet(object.distinctCount)
        ? Number(object.distinctCount)
        : 0,
    };
  },

  toJSON(message: ColumnStatistics): unknown {
    const obj: any = {};
    message.nullCount !== undefined &&
      (obj.nullCount = Math.round(message.nullCount));
    message.maxValue !== undefined && (obj.maxValue = message.maxValue);
    message.minValue !== undefined && (obj.minValue = message.minValue);
    message.distinctCount !== undefined &&
      (obj.distinctCount = Math.round(message.distinctCount));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ColumnStatistics>, I>>(
    object: I
  ): ColumnStatistics {
    const message = createBaseColumnStatistics();
    message.nullCount = object.nullCount ?? 0;
    message.maxValue = object.maxValue ?? "";
    message.minValue = object.minValue ?? "";
    message.distinctCount = object.distinctCount ?? 0;
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

function isObject(value: any): boolean {
  return typeof value === "object" && value !== null;
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
