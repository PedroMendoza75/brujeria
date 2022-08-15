pub mod brujeria_impl;
pub enum Ambito {
  Tonal{
    época: String,
    lado_izquierdo: &'static str, // parte práctica
    lado_derecho: &'static str, // parte mental
  },
  Nagual,
}
enum Direccion {
    Norte,
    Sur,
    Este,
    Oeste,
}
enum Intensidad {
    Sutil,
    Evidente,
    Abrumador,
}
enum Sensacion {
  Indeterminada,
  Temperatura(Intensidad),
  Humedad(Intensidad),
  Luz(Intensidad),
  Sonido(Intensidad),	
  Presión(Intensidad),
  Olor(Intensidad),
}

enum ParteCuerpo{
  Estómago,
  Pecho,
  Cabeza,
  Brazos,
  Pantorrillas,
  Canillas,
  Espalda
}
struct SensaciónCorporal{
  parte: ParteCuerpo,
  sensación: Sensacion,
}
enum Emoción{
  Fastidio(Intensidad),
  Miedo(Intensidad),
  Asco(Intensidad),
  Ira(Intensidad),
  Contento(Intensidad),
  Triste(Intensidad),
}
enum Vicio{
  Lujuria(Intensidad),
  Gula(Intensidad),
  Pereza(Intensidad),
  Envidia(Intensidad),
  Odio(Intensidad),
  Apego(Intensidad),
  Soberbia(Intensidad),
}
enum Disposición {
  Bocarriba, 
  Bocabajo, 
  Delado
}
 enum Postura {
  Tumbado (Disposición),
  Sentado,
  Encucliyado,
  Fetal,
  Depie,
 }
enum Elemento {
  Indefinido,
  Viento,
  Agua,
  Fuego,
  Tierra,
  Inorgánico,
}
struct Posición{
  dentro_fuera: i32,
  arriba_abajo: i32,
  izquierda_derecha: i32,
}
enum Practica {
  MarchaDePoder(Direccion) ,
  Recapitulacion,
  Acecho,
  Ensueño,
  SilencioInterno,
  LimpiarIslaTonal,
  PincheTirano,
  PararElMundo,
}
enum TipoEnergía {
    Natal,
    DeAlimentos,
    Sexual,
    Emocional,
    Mental,
    Fina
}
enum Aspecto {
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

enum Comportamiento {
    Estático,
    Vibratorio,
    Errático,
}

struct PuntoDeEncaje {
  posición: Posición,
  intensidad: Intensidad,
  comportamiento: Comportamiento
}
struct HuevoLuminoso {
  color: String,
  punto_de_encaje: PuntoDeEncaje,
  energia_predominante: TipoEnergía,
  cantidad_de_energia: i32,
  forma: String,
  compartimentos: i32
}

enum EtapaCamino{
  Inconciencia,
  Curiosidad,
  Aprendiz,
  Cazador,
  Guerrero,
  Vidente,
  HombreDeConocimiento,
}
enum TipoGuerrero{
  PorDefinir,
  Acechador,
  Ensoñador,
  Nagual
}
struct Conciencia{
  es_deliverada: bool,
  es_autoconciente: bool,
  aspecto: Aspecto,
}

struct Aprendizaje{
  augurio: String,
  práctica: Practica,
  objetivo: Aspecto,
  ambito: Ambito,
  contenido: String,
}

   
struct Guerrero {
  tipo: TipoGuerrero,
  energía: HuevoLuminoso,
  vicio: Vicio,
  predilección: String,
  etapa: EtapaCamino,
  aliado: Elemento,
  conciencia: Conciencia,
  aprendizaje: Aprendizaje
}
