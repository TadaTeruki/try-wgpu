/* tslint:disable */
/* eslint-disable */
/**
*/
export function start(): void;
/**
* @param {HTMLCanvasElement} canvas
* @param {boolean} use_gl_instead
* @returns {Promise<State | undefined>}
*/
export function create_state(canvas: HTMLCanvasElement, use_gl_instead: boolean): Promise<State | undefined>;
/**
*/
export class State {
  free(): void;
/**
* @param {KeyboardEvent} event
*/
  key_event(event: KeyboardEvent): void;
/**
*/
  leave(): void;
/**
*/
  scroll_to_right(): void;
/**
*/
  scroll_to_left(): void;
/**
* @param {number} _time
* @returns {Promise<void>}
*/
  update(_time: number): Promise<void>;
/**
* @param {number} width
* @param {number} height
*/
  resize(width: number, height: number): void;
/**
*/
  render(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly start: () => void;
  readonly create_state: (a: number, b: number) => number;
  readonly __wbg_state_free: (a: number, b: number) => void;
  readonly state_key_event: (a: number, b: number) => void;
  readonly state_leave: (a: number) => void;
  readonly state_scroll_to_right: (a: number) => void;
  readonly state_scroll_to_left: (a: number) => void;
  readonly state_update: (a: number, b: number) => number;
  readonly state_resize: (a: number, b: number, c: number) => void;
  readonly state_render: (a: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h5623f8b146cfcc3c: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hfb2befc4d0129c2b: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h00359a08e73a8bc6: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
