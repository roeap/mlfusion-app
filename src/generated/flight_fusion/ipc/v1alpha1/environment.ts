/* eslint-disable */
import * as _m0 from "protobufjs/minimal";

export interface ServiceConnection {
  host: string;
  port: number;
}

export interface EnvironmentInfo {
  serverVersion: string;
  artifactService: ServiceConnection | undefined;
  modelServing: ServiceConnection | undefined;
  dataSets: ServiceConnection | undefined;
  pipelinesUi: ServiceConnection | undefined;
  modelsUi: ServiceConnection | undefined;
}

function createBaseServiceConnection(): ServiceConnection {
  return { host: "", port: 0 };
}

export const ServiceConnection = {
  encode(
    message: ServiceConnection,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.host !== "") {
      writer.uint32(10).string(message.host);
    }
    if (message.port !== 0) {
      writer.uint32(16).int32(message.port);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ServiceConnection {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseServiceConnection();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.host = reader.string();
          break;
        case 2:
          message.port = reader.int32();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ServiceConnection {
    return {
      host: isSet(object.host) ? String(object.host) : "",
      port: isSet(object.port) ? Number(object.port) : 0,
    };
  },

  toJSON(message: ServiceConnection): unknown {
    const obj: any = {};
    message.host !== undefined && (obj.host = message.host);
    message.port !== undefined && (obj.port = Math.round(message.port));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ServiceConnection>, I>>(
    object: I
  ): ServiceConnection {
    const message = createBaseServiceConnection();
    message.host = object.host ?? "";
    message.port = object.port ?? 0;
    return message;
  },
};

function createBaseEnvironmentInfo(): EnvironmentInfo {
  return {
    serverVersion: "",
    artifactService: undefined,
    modelServing: undefined,
    dataSets: undefined,
    pipelinesUi: undefined,
    modelsUi: undefined,
  };
}

export const EnvironmentInfo = {
  encode(
    message: EnvironmentInfo,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.serverVersion !== "") {
      writer.uint32(10).string(message.serverVersion);
    }
    if (message.artifactService !== undefined) {
      ServiceConnection.encode(
        message.artifactService,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.modelServing !== undefined) {
      ServiceConnection.encode(
        message.modelServing,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.dataSets !== undefined) {
      ServiceConnection.encode(
        message.dataSets,
        writer.uint32(34).fork()
      ).ldelim();
    }
    if (message.pipelinesUi !== undefined) {
      ServiceConnection.encode(
        message.pipelinesUi,
        writer.uint32(42).fork()
      ).ldelim();
    }
    if (message.modelsUi !== undefined) {
      ServiceConnection.encode(
        message.modelsUi,
        writer.uint32(50).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): EnvironmentInfo {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEnvironmentInfo();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.serverVersion = reader.string();
          break;
        case 2:
          message.artifactService = ServiceConnection.decode(
            reader,
            reader.uint32()
          );
          break;
        case 3:
          message.modelServing = ServiceConnection.decode(
            reader,
            reader.uint32()
          );
          break;
        case 4:
          message.dataSets = ServiceConnection.decode(reader, reader.uint32());
          break;
        case 5:
          message.pipelinesUi = ServiceConnection.decode(
            reader,
            reader.uint32()
          );
          break;
        case 6:
          message.modelsUi = ServiceConnection.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): EnvironmentInfo {
    return {
      serverVersion: isSet(object.serverVersion)
        ? String(object.serverVersion)
        : "",
      artifactService: isSet(object.artifactService)
        ? ServiceConnection.fromJSON(object.artifactService)
        : undefined,
      modelServing: isSet(object.modelServing)
        ? ServiceConnection.fromJSON(object.modelServing)
        : undefined,
      dataSets: isSet(object.dataSets)
        ? ServiceConnection.fromJSON(object.dataSets)
        : undefined,
      pipelinesUi: isSet(object.pipelinesUi)
        ? ServiceConnection.fromJSON(object.pipelinesUi)
        : undefined,
      modelsUi: isSet(object.modelsUi)
        ? ServiceConnection.fromJSON(object.modelsUi)
        : undefined,
    };
  },

  toJSON(message: EnvironmentInfo): unknown {
    const obj: any = {};
    message.serverVersion !== undefined &&
      (obj.serverVersion = message.serverVersion);
    message.artifactService !== undefined &&
      (obj.artifactService = message.artifactService
        ? ServiceConnection.toJSON(message.artifactService)
        : undefined);
    message.modelServing !== undefined &&
      (obj.modelServing = message.modelServing
        ? ServiceConnection.toJSON(message.modelServing)
        : undefined);
    message.dataSets !== undefined &&
      (obj.dataSets = message.dataSets
        ? ServiceConnection.toJSON(message.dataSets)
        : undefined);
    message.pipelinesUi !== undefined &&
      (obj.pipelinesUi = message.pipelinesUi
        ? ServiceConnection.toJSON(message.pipelinesUi)
        : undefined);
    message.modelsUi !== undefined &&
      (obj.modelsUi = message.modelsUi
        ? ServiceConnection.toJSON(message.modelsUi)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<EnvironmentInfo>, I>>(
    object: I
  ): EnvironmentInfo {
    const message = createBaseEnvironmentInfo();
    message.serverVersion = object.serverVersion ?? "";
    message.artifactService =
      object.artifactService !== undefined && object.artifactService !== null
        ? ServiceConnection.fromPartial(object.artifactService)
        : undefined;
    message.modelServing =
      object.modelServing !== undefined && object.modelServing !== null
        ? ServiceConnection.fromPartial(object.modelServing)
        : undefined;
    message.dataSets =
      object.dataSets !== undefined && object.dataSets !== null
        ? ServiceConnection.fromPartial(object.dataSets)
        : undefined;
    message.pipelinesUi =
      object.pipelinesUi !== undefined && object.pipelinesUi !== null
        ? ServiceConnection.fromPartial(object.pipelinesUi)
        : undefined;
    message.modelsUi =
      object.modelsUi !== undefined && object.modelsUi !== null
        ? ServiceConnection.fromPartial(object.modelsUi)
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
