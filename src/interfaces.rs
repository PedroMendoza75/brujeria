use std::any::Any;

use enum_iterator::{next, previous};
use enum_unitary::EnumUnitary;

#[derive(Debug)]
pub enum ResultadoFluir {
  Plausible,
  Preocupante,
  Imposible,
}


pub trait Evoluciona {
  fn evolucionar(&mut self, aspecto: String);
  fn involucionar(&mut self, aspecto: String);
}
pub trait FluyeBase {
  fn avanzar(&mut self) -> ResultadoFluir {
    ResultadoFluir::Plausible
  }
  fn retroceder(&mut self) ->ResultadoFluir {
    ResultadoFluir::Plausible
  }
  fn as_any(&self) -> &dyn Any; 
}
pub trait Fluye: FluyeBase + EnumUnitary {
  fn ciclico() -> bool;
  fn avanzar(&mut self) -> ResultadoFluir {
      match next(self) {
        Some(aspecto_nuevo) => {
          *self = aspecto_nuevo;
          ResultadoFluir::Plausible
        }
        None => {
          if !Self::ciclico(){ 
            *self = Self::last().unwrap(); 
            ResultadoFluir::Imposible
          } else {
            *self = Self::first().unwrap();
            ResultadoFluir::Plausible
          }
        }
      }
    }
  fn retroceder(&mut self) ->ResultadoFluir {
      match previous(self) {
        Some(aspecto_nuevo) => {
          *self = aspecto_nuevo;
          ResultadoFluir::Plausible
        }
        None => {
          if !Self::ciclico(){
            *self = Self::first().unwrap();
             ResultadoFluir::Preocupante
          } else {
            *self = Self::last().unwrap(); 
            ResultadoFluir::Plausible
          }
        }
      }
    }
  }

// #[derive(Debug,PartialEq)]
// pub struct AspectoFluid<T> {
//   pub cíclico: bool,
//   pub aspecto:  Option< dyn enum_unitary:<T> >
// }

// impl AspectoFluido {
//   pub fn new(cíclico: bool, aspecto: dyn enum_unitary::EnumUnitary) -> AspectoFluido {
//     AspectoFluido {
//       cíclico,
//       aspecto:Some(aspecto)
//     }
//   }
// }