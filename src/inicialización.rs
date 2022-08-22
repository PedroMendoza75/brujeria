use std::collections::HashMap;

use crate::{enum_unit::*, interfaces::{Evoluciona}};
pub(crate) use super::estructuras::*;
use colored::Color;

//use enum_unitary::{EnumUnitary, FromPrimitive};
static PEDOS : &'static str = "que si que si, quesilete para tí la perra gorda";


impl Ambito{
  pub fn new () -> Ambito{
    Ambito::Tonal{
      época: "Occidental Informatizado".to_string(),
      lado_derecho: PEDOS,
      lado_izquierdo: PEDOS,
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
    
impl CuerpoEnergético  {
  pub fn new () -> CuerpoEnergético {
    CuerpoEnergético{
      color: Color::Red,
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

macro_rules! into_var { ($x:expr) => { $x.into().try_into().unwrap() } }
impl  Guerrero { // <T: enum_unitary::Unitary>
  // Constructor
  pub fn new(  
    vicio: Vicio, 
    conciencia: Conciencia, 
    aprendizaje: Aprendizaje) -> Guerrero {
      let mut g = Guerrero {
        energía: CuerpoEnergético::new(),
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

  pub fn compara_aspecto_fluido< T >( &self, key: &str, val: T ) ->Option< i8 > 
  where T: Copy + Into<usize> + 'static {
    let pos_v : i8 = into_var!( val );//.into().try_into().unwrap();
    match self.aspectos_fluidos.get(&key.to_string()) {
      Some(a) => match a.as_any().downcast_ref::< T >() {
          Some(b) => { 
            let pos_b : i8 = into_var!( *b );
            Some( pos_v - pos_b ) },//.into().try_into().unwrap();
          None => { println!("tipo no es del argumento" ); None }
      }
      None => { println!("clave no encontrada" ); None }
    }
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

