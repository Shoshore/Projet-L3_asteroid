use crate::asteroid::Asteroid;
use crate::stellarobject::StellarObject;
use macroquad::prelude::*;

/// Représente un missile dans le jeu.
/// Les missiles sont des objets stellaires qui se déplacent dans une direction fixe après leur lancement.
/// Ils peuvent détecter s'ils sont hors de l'écran ou s'ils ont été impliqués dans une collision.
pub struct Missile {
    /// Position actuelle du missile dans l'espace de jeu.
    position: Vec2,
    /// Vitesse du missile, définie par une direction et une magnitude.
    speed: Vec2,
    /// Indique si le missile a été impliqué dans une collision.
    has_collided: bool,
}

impl Missile {
    /// Crée un nouveau missile à une position donnée avec un angle de tir.
    ///
    /// # Arguments
    ///
    /// * `position` - La position initiale du missile.
    /// * `angle` - L'angle de tir du missile en radians.
    ///
    /// # Retour
    ///
    /// Une instance de `Missile`.
    pub fn new(position: Vec2, angle: f32) -> Self {
        Self {
            position,
            speed: Vec2::new(angle.sin() * 1.5, -angle.cos() * 1.5),
            has_collided: false,
        }
    }

    /// Vérifie si le missile est sorti de l'écran.
    ///
    /// # Arguments
    ///
    /// * `screen_width` - La largeur de l'écran.
    /// * `screen_height` - La hauteur de l'écran.
    ///
    /// # Retour
    ///
    /// `true` si le missile est hors de l'écran, sinon `false`.
    pub fn is_off_screen(&self, screen_width: f32, screen_height: f32) -> bool {
        self.position.x < 0.0
            || self.position.x > screen_width
            || self.position.y < 0.0
            || self.position.y > screen_height
    }

    /// Obtient l'état de collision du missile.
    ///
    /// # Retour
    ///
    /// `true` si le missile a été impliqué dans une collision, sinon `false`.
    pub fn get_collided(&self) -> bool {
        self.has_collided
    }
}

impl StellarObject for Missile {
    /// Obtient la position actuelle du missile.
    ///
    /// # Retour
    ///
    /// La position du missile sous forme de `Vec2`.
    fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Définit une nouvelle position pour le missile.
    ///
    /// # Arguments
    ///
    /// * `new_position` - La nouvelle position sous forme de `Vec2`.
    fn set_position(&mut self, new_position: Vec2) {
        self.position = new_position;
    }

    /// Obtient la vitesse actuelle du missile.
    ///
    /// # Retour
    ///
    /// La vitesse du missile sous forme de `Vec2`.
    fn get_speed(&self) -> Vec2 {
        self.speed
    }

    /// Définit une nouvelle vitesse pour le missile.
    ///
    /// # Arguments
    ///
    /// * `new_speed` - La nouvelle vitesse sous forme de `Vec2`.
    fn set_speed(&mut self, new_speed: Vec2) {
        self.speed = new_speed;
    }

    /// Met à jour la position du missile en fonction de sa vitesse.
    fn update_position(&mut self) {
        self.position += self.speed;
    }

    /// Gère une collision impliquant le missile.
    ///
    /// Marque le missile comme ayant été impliqué dans une collision.
    ///
    /// # Arguments
    ///
    /// * `_` - Le niveau de l'objet en collision (non utilisé ici).
    /// * `_` - Indique si la collision est confirmée (non utilisé ici).
    /// * `_` - Le vecteur de vitesse de l'objet en collision (non utilisé ici).
    ///
    /// # Retour
    ///
    /// Toujours `None` car un missile ne génère pas de nouveaux objets après une collision.
    fn handle_collision(&mut self, _: u8, _: bool, _: Vec2) -> Option<(Asteroid, Asteroid)> {
        self.has_collided = true;
        None
    }
}
