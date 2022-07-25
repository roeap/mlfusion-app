import { AreaSourceReference } from "../generated/flight_fusion/ipc/v1alpha1/common";
import { invoke } from "@tauri-apps/api";

export interface AreaInfo {
  source: AreaSourceReference;
}

export async function list_data_assets(): Promise<Array<AreaInfo>> {
  return invoke("list_data_assets", {});
}
