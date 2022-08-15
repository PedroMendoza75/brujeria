pub enum Ambito {
  Tonal{
    época: String,
    lado_izquierdo: &'static str, // parte práctica
    lado_derecho: &'static str, // parte mental
  },
  Nagual,
}
pub enum Direccion {
    Norte,
    Sur,
    Este,
    Oeste,
}
#[derive(Debug, PartialEq)]
pub enum Intensidad {
    Sutil,
    Evidente,
    Abrumador,
}
pub enum Sensacion {
  Indeterminada,
  Temperatura(Intensidad),
  Humedad(Intensidad),
  Luz(Intensidad),
  Sonido(Intensidad),	
  Presión(Intensidad),
  Olor(Intensidad),
}

pub enum ParteCuerpo{
  Estómago,
  Pecho,
  Cabeza,
  Brazos,
  Pantorrillas,
  Canillas,
  Espalda
}
pub struct SensaciónCorporal{
  pub parte: ParteCuerpo,
  pub sensación: Sensacion,
}
pub enum Emoción{
  Fastidio(Intensidad),
  Miedo(Intensidad),
  Asco(Intensidad),
  Ira(Intensidad),
  Contento(Intensidad),
  Triste(Intensidad),
}
#[derive(Debug, PartialEq)]
pub enum Vicio{
  Lujuria(Intensidad),
  Gula(Intensidad),
  Pereza(Intensidad),
  Envidia(Intensidad),
  Odio(Intensidad),
  Apego(Intensidad),
  Soberbia(Intensidad),
}
pub enum Disposición {
  Bocarriba, 
  Bocabajo, 
  Delado
}
pub enum Postura {
  Tumbado (Disposición),
  Sentado,
  Encucliyado,
  Fetal,
  Depie,
 }
 pub enum Elemento {
  Indefinido,
  Viento,
  Agua,
  Fuego,
  Tierra,
  Inorgánico,
}
pub struct Posición{
  pub dentro_fuera: i32,
  pub arriba_abajo: i32,
  pub izquierda_derecha: i32,
}
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
#[derive(Debug, PartialEq)]
pub enum TipoEnergía {
    Natal,
    DeAlimentos,
    Sexual,
    Emocional,
    Mental,
    Fina
}
pub enum Aspecto {
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

pub enum Comportamiento {
    Estático,
    Vibratorio,
    Errático,
}

pub struct PuntoDeEncaje {
  pub posición: Posición,
  pub intensidad: Intensidad,
  pub comportamiento: Comportamiento
}
pub struct HuevoLuminoso {
  pub color: String,
  pub punto_de_encaje: PuntoDeEncaje,
  pub energia_predominante: TipoEnergía,
  pub cantidad_de_energia: i32,
  pub forma: String,
  pub compartimentos: i32
}

pub enum EtapaCamino{
  Inconciencia,
  Curiosidad,
  Aprendiz,
  Cazador,
  Guerrero,
  Vidente,
  HombreDeConocimiento,
}
#[derive(Debug, PartialEq)]
pub enum TipoGuerrero{
  PorDefinir,
  Acechador,
  Ensoñador,
  Nagual
}

pub struct Conciencia{
  pub es_deliverada: bool,
  pub es_autoconciente: bool,
  pub aspecto: Aspecto,
}

pub struct Aprendizaje{
  pub augurio: String,
  pub práctica: Practica,
  pub objetivo: Aspecto,
  pub ambito: Ambito,
  pub contenido: String,
}

pub struct Guerrero {
  pub tipo: TipoGuerrero,
  pub energía: HuevoLuminoso,
  pub vicio: Vicio,
  pub predilección: String,
  pub etapa: EtapaCamino,
  pub aliado: Elemento,
  pub conciencia: Conciencia,
  pub aprendizaje: Aprendizaje
}
