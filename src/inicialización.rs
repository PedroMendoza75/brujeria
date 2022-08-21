use std::collections::HashMap;

use crate::{enum_unit::*, interfaces::{Evoluciona, FluyeBase}};
pub(crate) use super::estructuras::*;

static PEDOS : &'static str = "que si que si, quesilete para tí la perra gorda";


impl Ambito{
  pub fn new () -> Ambito{
    Ambito::Tonal{
      época: "Occidental Informatizado".to_string(),
      lado_izquierdo: PEDOS,
      lado_derecho: PEDOS,
    }
  }
}

impl Practica{
  pub fn new () -> Practica{
    Practica::MarchaDePoder(Direccion::Norte)
  }
}
  
impl Aprendizaje{
  pub fn new (
    augurio: &str,
  ) -> Aprendizaje{
    Aprendizaje{
      augurio: augurio.to_string(),
      práctica: Practica::new(), 
      objetivo: AspectoPersonal::new(),
      ambito: Ambito::new( ),
      contenido: String::new(),
    }
  }
}
impl Sensacion{
  pub fn new ( tipo: TipoSensacion ) -> Sensacion{
    Sensacion{
      tipo,
      intensidad : Intensidad::Sutil
    }
  }
}

impl AspectoPersonal {
  pub fn new () -> AspectoPersonal {
    AspectoPersonal::Fisico{
      sensaciones: SensaciónCorporal { 
        parte: ParteCuerpo::Estómago, 
        sensación: Sensacion::new(TipoSensacion::Presión) } ,
      postura: Postura::Tumbado(Disposición::Bocarriba),
      limites:(0,0),
      fuerza:0,
      }
  }
}

impl Posición{
  pub fn new () -> Posición{
    Posición{
      dentro_fuera:0,
      arriba_abajo:0,
      izquierda_derecha:0,      
    }
  }
}
    
impl HuevoLuminoso  {
  pub fn new () -> HuevoLuminoso {
    HuevoLuminoso{
      color: "blanco".to_string(),
      punto_de_encaje: PuntoDeEncaje{
        posición: Posición::new(),
        intensidad:  Intensidad::Sutil,
        comportamiento: Comportamiento::Estático,
      },
      energia_predominante: TipoEnergía::Sexual,
      cantidad_de_energia: 0,
      forma: "círculo".to_string(),
      compartimentos: 0,
    }
  }
}

impl Vicio{
  pub fn new () -> Vicio{
    Vicio{
      tipo: TipoVicio::Pereza,
      intensidad: Intensidad::Evidente
    }
    }
}

impl Conciencia {
  pub fn new () -> Conciencia{ 
    Conciencia { 
      es_deliverada: false,
      es_autoconciente: false, 
      aspecto: AspectoPersonal::new(),
    }
  }
}

// ("aliado",Elemento::new(false,Elemento::Tierra),
impl  Guerrero { // <T: enum_unitary::Unitary>
  // Constructor
  pub fn new(  
    vicio: Vicio, 
    conciencia: Conciencia, 
    aprendizaje: Aprendizaje) -> Guerrero {
      let mut g = Guerrero {
        energía: HuevoLuminoso::new(),
        vicio,
        predilección: "".to_string(),
        conciencia,
        aprendizaje,
        aspectos_fluidos: HashMap::new(),
      };
      g.aspectos_fluidos.insert("etapa".to_string(),Box::new(EtapaCamino::Inconciencia));
      g.aspectos_fluidos.insert("tipo".to_string(),Box::new(TipoGuerrero::PorDefinir));
      g
    }
  }
  impl Evoluciona for Guerrero { 
    fn evolucionar(&mut self, aspecto: String) {
      match self.aspectos_fluidos.get_mut(&aspecto) {
        Some(asp) => {
          asp.avanzar();
        }
        None => {
          println!("No se encontró el aspecto {}", aspecto);
        }
      }
    }
    fn involucionar(&mut self, aspecto: String) {
      match self.aspectos_fluidos.get_mut(&aspecto) {
        Some(asp) => {
          asp.retroceder();
        }
        None => {
          println!("No se encontró el aspecto {}", aspecto);
        }
      }
    }
  }

