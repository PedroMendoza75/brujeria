use std::collections::HashMap;

use crate::{enum_unit::*, enum_cmplx::*, interfaces::Evolución};
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

macro_rules! into_var { ($x:expr) => { $x.into().try_into() } }
impl  Guerrero { // <T: enum_unitary::Unitary>
  // Constructor
  pub fn new(  
    vicio: Vicio, 
    conciencia: Conciencia, 
    aprendizaje: Aprendizaje,
    arquetipo: Archetype) -> Guerrero {
      let mut g = Guerrero {
        energía: CuerpoEnergético::new(),
        vicio,
        predilección: "".to_string(),
        conciencia,
        aprendizaje,
        aspectos_fluidos: HashMap::new(),
        arquetipo
      };
      g.aspectos_fluidos.insert("etapa".to_string(),Box::new(EtapaCamino::Inconciencia));
      g.aspectos_fluidos.insert("tipo".to_string(),Box::new(TipoGuerrero::PorDefinir));
      g
    }

  /// Función que compara un aspecto fluido con un valor dado
  /// Devuelve una opción de i8, que es la diferencia entre la posición del valor en su enum correspondiente y la posición del aspecto fluido actual
  /// El Tipo de parámetro es un genérico que se debe implementar Copy, Into<usize> y 'static
  ///
  pub fn compara_aspecto_fluido<T>(&self, key: &str, val: T) -> Option<i8>
  where
      T: Copy + Into<usize> + 'static,
  {
      // Se obtiene la posición del valor en su enum correspondiente
      let pos_v_result: Result<i8, _> = into_var!(val);
      match pos_v_result {
          Ok(pos_v) => {
              // Se obtiene el aspecto fluido con la clave dada
              self.aspectos_fluidos.get(&key.to_string()).and_then(|a| {
                  // Se comprueba que el aspecto fluido es del tipo del valor
                  a.as_any().downcast_ref::<T>().and_then(|b| {
                      // Se obtiene la posición del aspecto fluido en su enum correspondiente
                      let pos_b_result: Result<i8, _> = into_var!(*b);
                      // Se envuelve la resta en Some() y se convierte Option<Option<i8>> a Option<i8>
                      pos_b_result.ok().map(|pos_b| Some(pos_v - pos_b)) 
                  }).flatten() 
              })
          },
          Err(_) => {
              // Si ocurre un error se devuelve None
              None
          },
      }
  }
}

// Implementar la trait Evoluciona para el struct Guerrero
impl Evolución for Guerrero { 
  // Función que evoluciona un aspecto fluido
  fn evolucionar(&mut self, aspecto: String) {
    // Buscar el aspecto fluido con la clave dada
    match self.aspectos_fluidos.get_mut(&aspecto) {
      // Si lo encuentra
      Some(asp) => {
        // Avanzar el aspecto fluido
        asp.avanzar();
      },
      // Si no lo encuentra
      None => {
        println!("No se encontró el aspecto fluido {}", aspecto);
      }
    }
  }
  // Función que retrocede un aspecto fluido
  fn involucionar(&mut self, aspecto: String) {
    // Buscar el aspecto fluido con la clave dada
    match self.aspectos_fluidos.get_mut(&aspecto) {
      // Si lo encuentra
      Some(asp) => {
        // Retroceder el aspecto fluido
        asp.retroceder();
      },
      // Si no lo encuentra
      None => {
        println!("No se encontró el aspecto fluido {}", aspecto);
      }
    }
  }
}

pub fn creacion_arquetipos() -> HashMap<Archetype, &'static str> {
  let mut map = HashMap::new();
  map.insert(Archetype::Innocent, "Soñadores ingenuos, siempre tratan de ver lo bueno en el mundo...");
  map.insert(Archetype::Sage, "Valora las ideas por encima de todo...");
  map.insert(Archetype::Explorer, "Necesita emociones nuevas...");
  map.insert(Archetype::Ruler, "Les gusta tener el control...");
  map.insert(Archetype::Magician, "Tienen una creencia verdadera en sus ideas...");
  map.insert(Archetype::Hero, "El héroe se esfuerza por defender a los demás...");
  map.insert(Archetype::Outlaw, "Les gusta hacer las cosas de manera diferente...");
  map.insert(Archetype::Lover, "Le resulta difícil lidiar con los conflictos...");
  map.insert(Archetype::Caregiver, "Se caracterizan por su empatía y compasión...");
  map.insert(Archetype::Jester, "Al bufón le encanta animar una fiesta con humor...");
  map.insert(Archetype::Everyperson, "Quieren ser parte de un todo mayor...");
  map.insert(Archetype::Child, "Vive en el presente y disfruta de las pequeñas cosas...");
  // map.remove(&Archetype::Innocent);
  map
}

