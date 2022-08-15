pub(crate) use crate::definiciones::*;

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
        objetivo: Aspecto::new(),
        ambito: Ambito::new( ),
        contenido: String::new(),
      }
    }
  }

  impl Aspecto {
    pub fn new () -> Aspecto {
      Aspecto::Fisico{
        sensaciones: SensaciónCorporal{ 
          parte: ParteCuerpo::Estómago, 
          sensación: Sensacion::Indeterminada } ,
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
          intensidad: Intensidad::Sutil,
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
      Vicio::Pereza(Intensidad::Evidente)
     }
  }

  impl Conciencia {
    pub fn new () -> Conciencia{ 
      Conciencia { 
        es_deliverada: false,
        es_autoconciente: false, 
        aspecto: Aspecto::new(),
      }
    }
  }

  impl Guerrero {
    // Constructor
    pub fn new(  
      vicio: Vicio, 
      conciencia: Conciencia, 
      aprendizaje: Aprendizaje) -> Guerrero {
        Guerrero {
          tipo: TipoGuerrero::PorDefinir,
          energía: HuevoLuminoso::new(),
          vicio,
          predilección: "".to_string(),
          etapa: EtapaCamino::Inconciencia,
          aliado: Elemento::Indefinido,
          conciencia,
          aprendizaje,
        }
      }
    }