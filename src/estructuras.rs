use super::enum_unit::*;
use super::enum_cmplx::*;
use crate::interfaces::FluyeBase;
use colored::Color;
use std::collections::HashMap;
#[derive(Debug)]
pub struct EmociónStr {
    pub tipo: Emoción,
    pub intensidad: Intensidad,
}

#[derive(Debug)]
pub struct Vicio {
    pub tipo: TipoVicio,
    pub intensidad: Intensidad,
}
#[derive(Debug)]
pub struct Posición {
    pub dentro_fuera: i32,
    pub arriba_abajo: i32,
    pub izquierda_derecha: i32,
}
#[derive(Debug)]
pub struct PuntoDeEncaje {
    pub posición: Posición,
    pub intensidad: Intensidad,
    pub comportamiento: Comportamiento,
}

pub struct CuerpoEnergético {
    pub color: Color,
    pub punto_de_encaje: PuntoDeEncaje,
    pub energia_predominante: TipoEnergía,
    pub cantidad_de_energia: i32,
    pub forma: String,
    pub compartimentos: i32,
}

pub struct Conciencia {
    pub es_deliverada: bool,
    pub es_autoconciente: bool,
    pub aspecto: AspectoPersonal,
}

pub struct Aprendizaje {
    pub augurio: String,
    pub práctica: Practica,
    pub objetivo: AspectoPersonal,
    pub ambito: Ambito,
    pub contenido: String,
}
pub struct Guerrero {
    pub energía: CuerpoEnergético,
    pub vicio: Vicio,
    pub predilección: String,
    pub conciencia: Conciencia,
    pub aprendizaje: Aprendizaje,
    pub aspectos_fluidos: HashMap<String, Box<dyn FluyeBase>>,
    pub arquetipo: Archetype,
}

pub struct Aventura {
    pub protagonista: Guerrero,
    pub reinos: Vec<Reino>,
}
pub struct Peripecia {
    pub descripcion: String,
    pub resultado: HashMap<ResultadoPeripecia, Box<Peripecia>>, // clave: resultado de la peripecia, valor: referencia a la siguiente peripecia
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ResultadoPeripecia {
    Éxito,
    Fracaso,
    Inconcluso,
}
pub struct Reino {
    pub nombre: String,
    pub climatologia: String,
    pub recursos: Vec<String>,
    pub fauna: Vec<String>,
    pub flora: Vec<String>,
    pub habitantes: Vec<String>,
    pub simbolos: Vec<String>,
    pub leyendas: Vec<String>,
    pub peripecias: HashMap<ResultadoPeripecia, Vec<Peripecia>>, // clave: resultado de la peripecia anterior, valor: array de posibles peripecias
}

impl Reino {
    pub fn new(nombre: String, climatologia: String, recursos: Vec<String>, fauna: Vec<String>, flora: Vec<String>, habitantes: Vec<String>, simbolos: Vec<String>, leyendas: Vec<String>) -> Reino {
        Reino {
            nombre,
            climatologia,
            recursos,
            fauna,
            flora,
            habitantes,
            simbolos,
            leyendas,
            peripecias: HashMap::new(),
        }
    }
}
impl Peripecia {
    pub fn new(descripcion: String, resultado: HashMap<ResultadoPeripecia, Box<Peripecia>>) -> Peripecia {
        Peripecia {
            descripcion,
            resultado,
        }
    }
}

