use enum_unitary::enum_unitary;
use std::any::Any;

use crate::interfaces::{Fluye,FluyeBase};
macro_rules! impl_fluye {
  ($x:ident, $s:expr) => {
    impl FluyeBase for $x {
      fn as_any(&self) -> &dyn Any{
        self
      }
    }
    impl Fluye for $x {
      fn ciclico() -> bool{$s}
    }
  }
}
enum_unitary! {
  #[derive(Debug, PartialEq)]
  pub enum Direccion {
    Sur,
    Este,
    Norte,
    Oeste
  }
}
enum_unitary! {
  #[derive(Debug, PartialEq)]
  pub enum Intensidad {
      Sutil,
      Evidente,
      Abrumador
  }
}
impl_fluye!(Intensidad, false);

#[derive(Debug)]
pub enum  TipoSensacion {
  Temperatura,
  Humedad,
  Luz,
  Sonido,	
  Presión,
  Olor,
}
enum_unitary! {
#[derive(Debug, PartialEq)]
  pub enum ParteCuerpo{
    Canillas,
    Pantorrillas,
    Estómago,
    Brazos,
    Espalda,
    Pecho,
    Cabeza
  }
}
impl_fluye!(ParteCuerpo, false);

#[derive(Debug)]
pub enum Emoción{
  Fastidio,
  Miedo,
  Asco,
  Ira,
  Contento,
  Triste,
}
#[derive(Debug, PartialEq)]
pub enum TipoVicio{
  Lujuria,
  Gula,
  Pereza,
  Envidia,
  Odio,
  Apego,
  Soberbia,
}
#[derive(Debug, PartialEq)]
pub enum Disposición {
  Bocarriba, 
  Bocabajo, 
  Delado
}
#[derive(Debug, PartialEq)]
pub enum Postura {
  Tumbado (Disposición),
  Sentado,
  Encucliyado,
  Fetal,
  Depie,
 }

 #[derive(Debug, PartialEq)]
 pub enum Elemento {
  Tierra,
  Agua,
  Fuego,
  Viento,
  Inorgánico,
}
#[derive(Debug, PartialEq)]
pub enum Practica {
  MarchaDePoder(Direccion) ,
  Recapitulacion,
  Acecho,
  Ensueño,
  SilencioInterno,
  LimpiarIslaTonal,
  PincheTirano,
  PararElMundo,
}
enum_unitary! {
#[derive(Debug, PartialEq)]
  pub enum TipoEnergía {
      Natal,
      DeAlimentos,
      Sexual,
      Emocional,
      Mental,
      Fina
  }
}
#[derive(Debug, PartialEq)]
pub enum Comportamiento {
    Estático,
    Vibratorio,
    Errático,
}

enum_unitary! {
#[derive(Debug, PartialEq)]
  pub enum EtapaCamino{
    Inconciencia,
    Curiosidad,
    Aprendiz,
    Cazador,
    Guerrero,
    Vidente,
    HombreDeConocimiento
  }
}
impl_fluye!(EtapaCamino, false);

enum_unitary! {
#[derive(Debug, PartialEq)]
  pub enum TipoGuerrero{
    PorDefinir,
    Acechador,
    Ensoñador,
    Nagual
  }
}
impl_fluye!(TipoGuerrero, false);

