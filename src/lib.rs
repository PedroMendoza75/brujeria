pub mod interfaces;
pub mod enum_unit;
pub mod enum_cmplx;
pub mod estructuras;
#[path ="inicialización.rs"]
pub mod inicialización;
#[cfg(test)]
mod tests {
    use crate::{
      enum_unit::*,
      interfaces::FluyeBase, 
      estructuras::{Guerrero, Vicio, Conciencia, Aprendizaje},
      inicialización::creacion_arquetipos
    };
    // use crate::estructuras::*;
  #[test]
  fn test_guerrero(){
    // let mut vc = Some( Intensidad::Evidente);
    // vc = enum_iterator::next( &vc.unwrap());
    struct AFF{
      pub fff: Box<dyn FluyeBase>
    }
    let mut af = Box::new(Intensidad::Sutil);
    let p2 = af.avanzar();
    print!("kk de la vka {:?} {:?} ", af, p2);
    let mut p3 = AFF{ fff : af};
    p3.fff.retroceder();
    
    let guerrero = Guerrero::new( 
      Vicio::new(),
      Conciencia::new(),
      Aprendizaje::new( "águila roza el coche"),
    Archetype::Innocent);
    
    println!("compara_aspecto_fluido:{:?}",
    guerrero.compara_aspecto_fluido("tipo", EtapaCamino::HombreDeConocimiento));

    let arqs = creacion_arquetipos();
    match arqs.get(&guerrero.arquetipo) {
        Some(description) => println!("Arquemostrico descr: {}", description),
        None => println!("no existe arquetipo para el guerrero"),
    }
    assert_eq!(guerrero.vicio.tipo, TipoVicio::Pereza);
    assert_eq!(guerrero.energía.energia_predominante, TipoEnergía::Sexual);
  }
}
// match guerrero.aspectos_fluidos.get("tipo") {

