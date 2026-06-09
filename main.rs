// PROYECTO: TAXI SOBERANO - BARCELONA AMB
// "Ayúdanos a regar la flor de la esperanza, desde nuestra humilde aportación"

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum TipoAviso {
    SosEmergencia,    // Alerta máxima (Auxilio en carretera)
    AvisoParada,      // Información útil de la jornada (Festivales, tráfico...)
}

#[derive(Debug, Clone)]
struct MensajeRed {
    id_emisor: String,       // Carnet IMET del taxista que avisa
    licencia_coche: String,  // Licencia vinculada para localizarlo
    tipo: TipoAviso,
    contenido: String,       // El texto del aviso
    hora_registro: String,   // Para saber cuándo se lanzó
}

#[derive(Debug, Clone)]
struct NodoTaxi {
    carnet_imet: String,       // El carnet personal del taxista (IMET/AMB)
    licencia_vinculada: String, // La licencia del coche que trabaja este turno
    tokens_privados: f64,       // Billetes TEU disponibles
    aportacion_social: f64,     // Lo que este nodo siembra para las ayudas de Barcelona
    esta_bloqueado: bool,
}

// =========================================================================
// NUEVA ESTRUCTURA DEL PROTOCOLO DE APERTURA (LA QUE TE COMENTABA)
// =========================================================================
#[derive(Debug, Clone)]
struct SolicitudIngreso {
    carnet_nuevo: String,
    licencia_nueva: String,
    aval_vicente: bool,   // Tu firma como enlace de la calle
    aval_jefe: bool,      // La firma de tu jefe como Guardián IMET
}

// La Red que engloba a todos los conductores de la ciudad
struct RedTaxiSoberano {
    nodos: HashMap<String, NodoTaxi>,
    tablon_avisos: Vec<MensajeRed>,
    solicitudes_pendientes: HashMap<String, SolicitudIngreso>, // El libro de registro de aspirantes
}

impl RedTaxiSoberano {
    fn new() -> Self {
        RedTaxiSoberano { 
            nodos: HashMap::new(),
            tablon_avisos: Vec::new(),
            solicitudes_pendientes: HashMap::new(),
        }
    }

    // Registrar al taxista fundador de forma individualizada
    fn registrar_taxista(&mut self, taxi: NodoTaxi) {
        self.nodos.insert(taxi.carnet_imet.clone(), taxi);
    }

    // 1. PROTOCOLO: El nuevo compañero pide entrar a la Red de Honor
    fn registrar_solicitud_ingreso(&mut self, carnet: &str, licencia: &str) {
        let nueva_solicitud = SolicitudIngreso {
            carnet_nuevo: carnet.to_string(),
            licencia_nueva: licencia.to_string(),
            aval_vicente: false,
            aval_jefe: false,
        };
        self.solicitudes_pendientes.insert(carnet.to_string(), nueva_solicitud);
        println!("📝 PROTOCOLO: Solicitud registrada para el Carnet [{}]. Esperando ciclo de apertura Vicente/Jefe...", carnet);
    }

    // 2. PROTOCOLO: TÚ Y TU JEFE FIRMÁIS EL AVAL EN LA PARADA
    fn firmar_aval_ingreso(&mut self, carnet_aspirante: &str, carnet_avalista: &str) -> Result<String, String> {
        if let Some(solicitud) = self.solicitudes_pendientes.get_mut(carnet_aspirante) {
            
            if carnet_avalista == "IMET-5800" {
                solicitud.aval_vicente = true;
                println!("✍️ Vicente (IMET-5800) ha firmado el aval de honor para [{}].", carnet_aspirante);
            } else if carnet_avalista == "IMET-0001" {
                solicitud.aval_jefe = true;
                println!("🛡️ El Jefe-Guardián (IMET-0001) ha validado los datos IMET para [{}].", carnet_aspirante);
            } else {
                return Err("🔴 RECHAZADO: No tienes permisos de avalista de apertura en este turno.".to_string());
            }

            // SI AMBOS HABÉIS FIRMADO, SE PRODUCE LA APERTURA AUTOMÁTICA
            if solicitud.aval_vicente && solicitud.aval_jefe {
                let nuevo_nodo = NodoTaxi {
                    carnet_imet: solicitud.carnet_nuevo.clone(),
                    licencia_vinculada: solicitud.licencia_nueva.clone(),
                    tokens_privados: 500.0, // Carga inicial de confianza
                    aportacion_social: 0.0,
                    esta_bloqueado: false,
                };
                
                self.nodos.insert(nuevo_nodo.carnet_imet.clone(), nuevo_nodo);
                self.solicitudes_pendientes.remove(carnet_aspirante); // Sacamos de la lista de espera
                
                Ok(format!(
                    "🟢 ¡CICLO DE APERTURA COMPLETADO! El carnet [{}] (Licencia {}) ya es un NODO OFICIAL de la Red de Honor.",
                    carnet_aspirante, solicitud.licencia_nueva
                ))
            } else {
                Ok(format!("⏳ Solicitud de [{}] en espera de la segunda firma del ciclo.", carnet_aspirante))
            }
        } else {
            Err("🔴 ERROR: No se encontró ninguna solicitud activa para ese carnet.".to_string())
        }
    }

    // EL ESCUDO DE CONSENSO: Procesar gastos con la firma presencial del Guardián
    fn procesar_gasto_con_consenso(
        &mut self,
        carnet_solicitante: &str,
        carnet_guardian: &str,
        importe: f64,
        firma_solicitante: bool,
        firma_guardian: bool,
    ) -> Result<String, String> {
        if !firma_solicitante || !firma_guardian {
            if let Some(taxista) = self.nodos.get_mut(carnet_solicitante) {
                taxista.esta_bloqueado = true;
            }
            return Err(format!("🚨 ALERTA: Intento de fraude. El carnet IMET [{}] ha sido BLOQUEADO.", carnet_solicitante));
        }
        if !self.nodos.contains_key(carnet_guardian) {
            return Err(format!("🔴 RECHAZADO: El carnet guardián [{}] no consta en el sistema.", carnet_guardian));
        }
        if let Some(taxista) = self.nodos.get_mut(carnet_solicitante) {
            if taxista.esta_bloqueado {
                return Err("🔴 RECHAZADO: Este carnet está suspendido por la red.".to_string());
            }
            if taxista.tokens_privados >= importe {
                taxista.tokens_privados -= importe;
                Ok(format!("🟢 CONSENSO: Taxista [{}] (Licencia {}) autoriza {} TEU.", carnet_solicitante, taxista.licencia_vinculada, importe))
            } else {
                Err("🔴 RECHAZADO: Fondos insuficientes.".to_string())
            }
        } else {
            Err("🔴 RECHAZADO: No registrado.".to_string())
        }
    }
}

fn main() {
    println!("--- CONFIGURACIÓN DE PROTOCOLOS DE APERTURA ---");
    let mut red_amb = RedTaxiSoberano::new();

    // 1. Os damos de alta a ti y a tu jefe como los pilares autorizados de Barcelona
    red_amb.registrar_taxista(NodoTaxi {
        carnet_imet: "IMET-5800".to_string(), // Tú, Vicente
        licencia_vinculada: "AMB-1234".to_string(),
        tokens_privados: 1000.0,
        aportacion_social: 0.0,
        esta_bloqueado: false,
    });
    red_amb.registrar_taxista(NodoTaxi {
        carnet_imet: "IMET-0001".to_string(), // Tu Jefe
        licencia_vinculada: "AMB-3931".to_string(),
        tokens_privados: 3000.0,
        aportacion_social: 600.0,
        esta_bloqueado: false,
    });

    println!("Nodos fundadores listos. Esperando solicitudes...\n");

    // 2. Un compañero nuevo solicita entrar desde su móvil
    red_amb.registrar_solicitud_ingreso("IMET-7777", "AMB-5555");
    println!("--------------------------------------------------");

    // 3. FASE 1 DEL CICLO: Tú estás con él en la parada, das el visto bueno y firmas
    println!("Fase 1: Vicente pone su firma de honor...");
    if let Ok(msg) = red_amb.firmar_aval_ingreso("IMET-7777", "IMET-5800") {
        println!("{}", msg);
    }
    
    // Comprobamos si ya puede operar (Debe decir false, falta el jefe)
    println!("¿El nuevo compañero está en la red activa? {}", red_amb.nodos.contains_key("IMET-7777"));
    println!("--------------------------------------------------");

    // 4. FASE 2 DEL CICLO: Tu jefe mete la firma institucional
    println!("Fase 2: El Jefe de la parada introduce la firma institucional...");
    if let Ok(msg) = red_amb.firmar_aval_ingreso("IMET-7777", "IMET-0001") {
        println!("{}", msg);
    }

    // Comprobación final: El ciclo de apertura funciona (Debe decir true)
    println!("--------------------------------------------------");
    println!("¿Compañero activo de forma soberana? {}", red_amb.nodos.contains_key("IMET-7777"));
}
