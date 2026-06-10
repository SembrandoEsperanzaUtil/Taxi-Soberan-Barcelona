// PROYECTO: TAXI SOBERANO - BARCELONA AMB
// "Barcelona, això és TEU! Món, això és TEU!"

use std::collections::HashMap;
use wasm_bindgen::prelude::*; // 🔌 El puente de acero que conecta Rust con el móvil móvil (WebAssembly)

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
enum TipoAviso {
    SosEmergencia,
    AvisoParada,
}

#[derive(Debug, Clone, PartialEq)]
enum TipoAviso {
    SosEmergencia,    // Alerta máxima (Auxilio en carretera)
    AvisoParada,      // Información útil de la jornada (Festivales, tráfico...)
}
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct MensajeRed {
    id_emisor: String,
    licencia_coche: String,
    tipo: TipoAviso,
    contenido: String,
    hora_registro: String,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct NodoTaxi {
    carnet_imet: String,
    licencia_vinculada: String,
    tokens_privados: f64,
    aportacion_social: f64,
    esta_bloqueado: bool,
}

#[derive(Debug, Clone)]
struct SolicitudIngreso {
    carnet_nuevo: String,
    licencia_nueva: String,
    aval_vicente: bool,
    aval_jefe: bool,
}

// LA RED SOBERANA CONECTADA DIRECTAMENTE AL MÓVIL
#[wasm_bindgen]
pub struct RedTaxiSoberano {
    nodos: HashMap<String, NodoTaxi>,
    tablon_avisos: Vec<MensajeRed>,
    solicitudes_pendientes: HashMap<String, SolicitudIngreso>,
}

#[wasm_bindgen]
impl RedTaxiSoberano {
    // Constructor público para que el navegador web inicie la red
    #[wasm_bindgen(constructor)]
    pub fn new() -> RedTaxiSoberano {
        RedTaxiSoberano { 
            nodos: HashMap::new(),
            tablon_avisos: Vec::new(),
            solicitudes_pendientes: HashMap::new(),
        }
    }

    // Registrar los conductores fundadores desde la web
    pub fn registrar_taxista(&mut self, carnet: String, licencia: String, tokens: f64) {
        let taxi = NodoTaxi {
            carnet_imet: carnet.clone(),
            licencia_vinculada: licencia,
            tokens_privados: tokens,
            aportacion_social: 0.0,
            esta_bloqueado: false,
        };
        self.nodos.insert(carnet, taxi);
    }

    // 1. PROTOCOLO: El aspirante pide pista desde su terminal móvil
    pub fn registrar_solicitud_ingreso(&mut self, carnet: &str, licencia: &str) {
        let nueva_solicitud = SolicitudIngreso {
            carnet_nuevo: carnet.to_string(),
            licencia_nueva: licencia.to_string(),
            aval_vicente: false,
            aval_jefe: false,
        };
        self.solicitudes_pendientes.insert(carnet.to_string(), nueva_solicitud);
    }

    // 2. PROTOCOLO: El ciclo de apertura Vicente + Jefe en directo
    pub fn firmar_aval_ingreso(&mut self, carnet_aspirante: &str, carnet_avalista: &str) -> String {
        if let Some(solicitud) = self.solicitudes_pendientes.get_mut(carnet_aspirante) {
            if carnet_avalista == "IMET-5800" {
                solicitud.aval_vicente = true;
            } else if carnet_avalista == "IMET-0001" {
                solicitud.aval_jefe = true;
            } else {
                return "🔴 RECHAZADO: Permisos de avalista denegados.".to_string();
            }

            if solicitud.aval_vicente && solicitud.aval_jefe {
                let nuevo_nodo = NodoTaxi {
                    carnet_imet: solicitud.carnet_nuevo.clone(),
                    licencia_vinculada: solicitud.licencia_nueva.clone(),
                    tokens_privados: 500.0, // Carga inicial de confianza útil
                    aportacion_social: 0.0,
                    esta_bloqueado: false,
                };
                self.nodos.insert(nuevo_nodo.carnet_imet.clone(), nuevo_nodo);
                self.solicitudes_pendientes.remove(carnet_aspirante);
                format!("🟢 CICLO DE APERTURA COMPLETADO: Carnet [{}] dado de alta soberana.", carnet_aspirante)
            } else {
                format!("⏳ Solicitud retenida. Esperando la otra firma del ciclo para el carnet [{}].", carnet_aspirante)
            }
        } else {
            "🔴 ERROR: No hay solicitud activa para ese carnet.".to_string()
        }
    }

    // EL ESCUDO DE CONSENSO CON DOBLE VALIDACIÓN DIGITAL
    pub fn procesar_gasto_con_consenso(
        &mut self,
        carnet_solicitante: &str,
        carnet_guardian: &str,
        importe: f64,
        firma_solicitante: bool,
        firma_guardian: bool,
    ) -> String {
        if !firma_solicitante || !firma_guardian {
            if let Some(taxista) = self.nodos.get_mut(carnet_solicitante) {
                taxista.esta_bloqueado = true;
            }
            return format!("🚨 FRAUDE DETECTADO: El carnet [{}] ha sido BLOQUEADO.", carnet_solicitante);
        }
        if !self.nodos.contains_key(carnet_guardian) {
            return format!("🔴 RECHAZADO: El Guardián [{}] no consta en la red.", carnet_guardian);
        }
        if let Some(taxista) = self.nodos.get_mut(carnet_solicitante) {
            if taxista.esta_bloqueado {
                return "🔴 RECHAZADO: Este carnet está suspendido.".to_string();
            }
            if taxista.tokens_privados >= importe {
                taxista.tokens_privados -= importe;
                format!("🟢 CONSENSO LOGRADO: Gasto de {} TEU autorizado por el Guardián.", importe)
            } else {
                "🔴 RECHAZADO: Fondos insuficientes en el Bolsillo Privado.".to_string()
            }
        } else {
            "🔴 RECHAZADO: Conductor no registrado.".to_string()
        }
    }

    // BOTÓN DE AUXILIO SOS MULTIDIFUSIÓN
    pub fn lanzar_alerta_sos(&mut self, carnet_emisor: &str, contenido: &str, hora: &str) -> String {
        if let Some(taxista) = self.nodos.get(carnet_emisor) {
            let alerta = MensajeRed {
                id_emisor: carnet_emisor.to_string(),
                licencia_coche: taxista.licencia_vinculada.clone(),
                tipo: TipoAviso::SosEmergencia,
                contenido: contenido.to_string(),
                hora_registro: hora.to_string(),
            };
            self.tablon_avisos.push(alerta);
            format!("🚨 SOS EMITIDO desde la Licencia {}. Solicitando auxilio inmediato...", taxista.licencia_vinculada)
        } else {
            "🔴 ERROR: Emisor no identificado.".to_string()
        }
    }
} // <--- ¡Esta es la ultimísima llave de todo tu archivo!
