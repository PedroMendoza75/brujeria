pub mod definiciones;
#[path ="inicialización.rs"]
pub mod inicialización;
#[cfg(test)]
mod tests {
    use crate::definiciones::*;

  #[test]
  fn test_guerrero(){
    let guerrero = Guerrero::new( Vicio::Pereza(Intensidad::Evidente),
                                      Conciencia::new(),
                                      Aprendizaje::new( "águila roza el coche"));
    assert_eq!(guerrero.tipo, TipoGuerrero::PorDefinir);
    assert_eq!(guerrero.vicio, Vicio::Pereza(Intensidad::Evidente));
    assert_eq!(guerrero.energía.energia_predominante, TipoEnergía::Sexual);
  }
}