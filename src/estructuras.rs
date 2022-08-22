use std::collections::HashMap;
use crate::interfaces::FluyeBase;
use colored::Color;
use super::enum_unit::*;

pub enum Ambito {
  Tonal{
    época: String,
    lado_izquierdo: &'static str, // parte práctica
    lado_derecho: &'static str, // parte mental
  },
  Nagual,
}

#[derive(Debug)]
pub enum AspectoPersonal {
  Fisico  {
    sensaciones: SensaciónCorporal,
    postura: Postura,
    limites: (i32,i32),
    fuerza: i32,
  },
  Emocional(Emoción),
  Mental(String),
  Energético(TipoEnergía),
}

#[derive(Debug)]
pub struct Sensacion {
  pub tipo: TipoSensacion,
  pub intensidad: Intensidad,
}
#[derive(Debug)]
pub struct SensaciónCorporal{
  pub parte: ParteCuerpo,
  pub sensación: Sensacion,
}

#[derive(Debug)]
pub struct EmociónStr{
  pub tipo: Emoción,
  pub intensidad: Intensidad,
}

#[derive(Debug)]
pub struct Vicio{
  pub tipo: TipoVicio,
  pub intensidad: Intensidad,
}
#[derive(Debug)]
pub struct Posición{
  pub dentro_fuera: i32,
  pub arriba_abajo: i32,
  pub izquierda_derecha: i32,
}
#[derive(Debug)]
pub struct PuntoDeEncaje {
  pub posición: Posición,
  pub intensidad: Intensidad,
  pub comportamiento: Comportamiento
}

pub struct CuerpoEnergético {
  pub color: Color,
  pub punto_de_encaje: PuntoDeEncaje,
  pub energia_predominante: TipoEnergía,
  pub cantidad_de_energia: i32,
  pub forma: String,
  pub compartimentos: i32
}

pub struct Conciencia{
  pub es_deliverada: bool,
  pub es_autoconciente: bool,
  pub aspecto: AspectoPersonal,
}

pub struct Aprendizaje{
  pub augurio: String,
  pub práctica: Practica,
  pub objetivo: AspectoPersonal,
  pub ambito: Ambito,
  pub contenido: String,
}
pub struct Guerrero{
  pub energía: CuerpoEnergético,
  pub vicio: Vicio,
  pub predilección: String,
  pub conciencia: Conciencia,
  pub aprendizaje: Aprendizaje,
  pub aspectos_fluidos: HashMap<String, 
                        Box< dyn FluyeBase > >
}
