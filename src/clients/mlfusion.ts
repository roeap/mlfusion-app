import { AreaSourceReference, AreaSourceDetails } from "../generated";
import { invoke } from "@tauri-apps/api";

export interface AreaInfo {
  source: AreaSourceReference;
}

export async function list_data_assets(): Promise<Array<AreaInfo>> {
  return invoke("list_data_assets", {});
}

export async function get_data_asset_info(
  source: AreaSourceReference
): Promise<AreaSourceDetails> {
  return invoke("get_data_asset_info", { source });
}
