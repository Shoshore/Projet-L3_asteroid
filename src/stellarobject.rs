use crate::asteroid::Asteroid;
use macroquad::prelude::Vec2;

/// Un trait représentant un objet stellaire dans le jeu.
/// Ce trait définit les comportements de base pour tous les objets stellaires,
/// tels que les astéroïdes, les missiles ou les vaisseaux spatiaux.
/// Il inclut des fonctionnalités pour gérer la position, la vitesse, la mise à jour
/// des mouvements et les collisions avec d'autres objets.
pub trait StellarObject {
    /// Obtient la position actuelle de l'objet stellaire.
    ///
    /// # Retour
    ///
    /// La position de l'objet sous forme d'un vecteur `Vec2`.
    fn get_position(&self) -> Vec2;

    /// Définit une nouvelle position pour l'objet stellaire.
    ///
    /// # Arguments
    ///
    /// * `new_position` - La nouvelle position sous forme de `Vec2`.
    fn set_position(&mut self, new_position: Vec2);

    /// Obtient la vitesse actuelle de l'objet stellaire.
    ///
    /// # Retour
    ///
    /// La vitesse de l'objet sous forme d'un vecteur `Vec2`.
    fn get_speed(&self) -> Vec2;

    /// Définit une nouvelle vitesse pour l'objet stellaire.
    ///
    /// # Arguments
    ///
    /// * `new_speed` - La nouvelle vitesse sous forme de `Vec2`.
    fn set_speed(&mut self, new_speed: Vec2);

    /// Met à jour la position de l'objet stellaire.
    ///
    /// Cette méthode applique la vitesse actuelle à la position de l'objet
    /// et peut inclure des ajustements pour gérer les limites de l'écran ou d'autres règles.
    fn update_position(&mut self);

    /// Gère une collision impliquant l'objet stellaire.
    ///
    /// Cette méthode est appelée lorsqu'une collision est détectée entre l'objet
    /// stellaire et un autre. Par exemple, lorsqu'un astéroïde entre en collision
    /// avec un missile ou un vaisseau.
    ///
    /// # Arguments
    ///
    /// * `asteroid_level` - Le niveau de l'astéroïde impliqué dans la collision
    ///                      (utilisé pour calculer les effets de la collision).
    /// * `status` - Un indicateur booléen pour signaler si une collision s'est produite.
    /// * `speed_missile` - La vitesse de l'objet impliqué dans la collision (par exemple, un missile).
    ///
    /// # Retour
    ///
    /// Une option contenant un tuple de deux nouveaux astéroïdes si une division est requise,
    /// sinon `None`.
    fn handle_collision(
        &mut self,
        asteroid_level: u8,
        status: bool,
        speed_missile: Vec2,
    ) -> Option<(Asteroid, Asteroid)>;
}
