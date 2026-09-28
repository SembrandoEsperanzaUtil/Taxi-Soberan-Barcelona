/* tslint:disable */
/* eslint-disable */

export class MensajeRed {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

export class NodoTaxi {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
}

export class RedTaxiSoberano {
    free(): void;
    [Symbol.dispose](): void;
    firmar_aval_ingreso(carnet_aspirante: string, carnet_avalista: string): string;
    lanzar_alerta_sos(carnet_emisor: string, contenido: string, hora: string): string;
    constructor();
    procesar_gasto_con_consenso(carnet_solicitante: string, carnet_guardian: string, importe: number, firma_solicitante: boolean, firma_guardian: boolean): string;
    registrar_solicitud_ingreso(carnet: string, licencia: string): void;
    registrar_taxista(carnet: string, licencia: string, tokens: number): void;
    votar_censura_comunitaria(carnet_infractor: string, carnet_votante: string): string;
}

export enum TipoAviso {
    SosEmergencia = 0,
    AvisoParada = 1,
}

export function emitir_aviso(tipo: string, ubicacion: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_mensajered_free: (a: number, b: number) => void;
    readonly __wbg_nodotaxi_free: (a: number, b: number) => void;
    readonly __wbg_redtaxisoberano_free: (a: number, b: number) => void;
    readonly emitir_aviso: (a: number, b: number, c: number, d: number) => [number, number];
    readonly redtaxisoberano_firmar_aval_ingreso: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly redtaxisoberano_lanzar_alerta_sos: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
    readonly redtaxisoberano_new: () => number;
    readonly redtaxisoberano_procesar_gasto_con_consenso: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
    readonly redtaxisoberano_registrar_solicitud_ingreso: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly redtaxisoberano_registrar_taxista: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly redtaxisoberano_votar_censura_comunitaria: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
