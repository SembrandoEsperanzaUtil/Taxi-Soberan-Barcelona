// PROYECTO: TAXI SOBERANO - BARCELONA AMB
// "Ayúdanos a regar la flor de la esperanza, desde nuestra humilde aportación"

use std::collections::HashMap;

#[derive(Debug)]
struct NodoTaxi {
    licencia: String,
    ahorro_tokens: f64,
    guardian_asignado: String, // ID del compañero de confianza (ej. el Jefe)
    esta_bloqueado: bool,
}

impl NodoTaxi {
    // Función para liberar fondos (Seguro, Talleres, Combustible)
    fn autorizar_gasto(
        &mut self, 
        importe: f64, 
        firma_biometrica_propia: bool, 
        firma_guardian: bool
    ) -> Result<String, String> {
        
        // El escudo contra "bandidos" remotos: se requieren ambas presencias físicas
        if firma_biometrica_propia && firma_guardian {
            if self.ahorro_tokens >= importe {
                self.ahorro_tokens -= importe;
                Ok(format!("Pago de {} tokens autorizado con éxito. La unión hace la fuerza.", importe))
            } else {
                Err("Fondos insuficientes en el Token de la Esperanza.".to_string())
            }
        } else {
            Err("ALERTA: Intento de acceso sin doble validación física. Bloqueando fondos.".to_string())
        }
    }
}

fn main() {
    println!("--- Iniciando Nodo de Honor: Taxi Barcelona ---");
    
    // Ejemplo de uso para Vicente y su Jefe
    let mut taxi_vicente = NodoTaxi {
        licencia: "AMB-1234".to_string(),
        ahorro_tokens: 1000.0,
        guardian_asignado: "Jefe-Nodo-01".to_string(),
        esta_bloqueado: false,
    };

    println!("Estado actual: {:?}", taxi_vicente);
}
