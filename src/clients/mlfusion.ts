import { AreaSourceReference, AreaSourceMetadata } from "../generated";
import { invoke } from "@tauri-apps/api";

export interface AreaInfo {
  source: AreaSourceReference;
}

export interface AreaDetails {
  meta: AreaSourceMetadata;
}

export async function list_data_assets(): Promise<Array<AreaInfo>> {
  return invoke("list_data_assets", {});
}

export async function get_data_asset_info(): Promise<
  Array<AreaSourceMetadata>
> {
  return invoke("get_data_asset_info", {});
}
