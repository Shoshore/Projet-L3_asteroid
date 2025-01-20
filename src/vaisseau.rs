use crate::asteroid::Asteroid;
use crate::missile::Missile;
use crate::stellarobject::StellarObject;
use macroquad::prelude::*;

/// Représente un Vaisseau contrôlé par le joueur.
/// Le vaisseau peut se déplacer, tirer des missiles, et subir des dégâts lorsqu'il entre en collision
/// avec des astéroïdes. Il possède également un bouclier pour encaisser les dégâts.
pub struct Vaisseau {
    /// Position actuelle de le vaisseau dans l'espace de jeu.
    position: Vec2,
    /// Angle de rotation de le vaisseau (en radians).
    rotation: f32,
    /// Vecteur de vitesse de le vaisseau.
    speed: Vec2,
    /// Points de bouclier restant de le vaisseau.
    shield: f32,
    /// Heure du dernier tir (en secondes depuis le début de l'exécution).
    last_shot: f64,
}

impl Vaisseau {
    /// Crée une nouvelle instance de `Vaisseau` avec des paramètres par défaut.
    ///
    /// Le vaisseau démarre au centre de l'écran, sans rotation, avec une vitesse nulle, un bouclier
    /// de 5 points, et un temps initial pour le dernier tir.
    ///
    /// # Retour
    ///
    /// Une instance de `Vaisseau`.
    pub fn new(position: Option<Vec2>, last_shot: Option<f64>) -> Self {
        Self {
            position: position.unwrap_or_else(|| vec2(screen_width() / 2., screen_height() / 2.)),
            rotation: 0.,
            speed: Vec2::new(0., 0.),
            shield: 5.,
            last_shot: last_shot.unwrap_or_else(get_time),
        }
    }

    /// Retourne le nombre de points de bouclier restant de le vaisseau.
    ///
    /// # Retour
    ///
    /// Nombre de points de bouclier.
    pub fn get_shield(&self) -> f32 {
        self.shield
    }

    /// Réduit les points de bouclier de le vaisseau.
    ///
    /// # Arguments
    ///
    /// * `dmg` - Nombre de points de dégâts à soustraire.
    pub fn dmg_shield(&mut self, dmg: f32) {
        self.shield -= dmg;
    }

    /// Retourne l'angle actuel de rotation de l'vaisseau.
    ///
    /// # Retour
    ///
    /// L'angle de rotation en radians.
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    /// Tente de tirer un missile si le temps de recharge est écoulé.
    ///
    /// # Arguments
    ///
    /// * `current_time` - Temps actuel (en secondes depuis le début de l'exécution).
    ///
    /// # Retour
    ///
    /// Une instance de `Missile` si le tir est possible, sinon `None`.
    pub fn fire_missile(&mut self, current_time: f64) -> Option<Missile> {
        if is_key_down(KeyCode::Space) && (current_time - self.last_shot >= 0.5) {
            self.last_shot = current_time;
            Some(Missile::new(self.position, self.rotation))
        } else {
            None
        }
    }

    /// Contraint la position de l'vaisseau à rester à l'intérieur des limites de l'écran.
    ///
    /// Si la position dépasse les limites, elle est ramenée de l'autre côté de l'écran (effet "wrap-around").
    ///
    /// # Arguments
    ///
    /// * `pos` - La position à contraindre.
    ///
    /// # Retour
    ///
    /// Une nouvelle position contrainte dans les limites de l'écran.
    fn bound_position(pos: Vec2) -> Vec2 {
        Vec2::new(
            Self::wrap_position(pos.x, screen_width()),
            Self::wrap_position(pos.y, screen_height()),
        )
    }

    /// Applique l'effet "wrap-around" sur une coordonnée donnée.
    ///
    /// Si la coordonnée dépasse les limites spécifiées, elle est ajustée pour revenir de l'autre côté.
    ///
    /// # Arguments
    ///
    /// * `coord` - La coordonnée à ajuster.
    /// * `max` - La limite supérieure pour la coordonnée.
    ///
    /// # Retour
    ///
    /// La coordonnée ajustée.
    fn wrap_position(coord: f32, max: f32) -> f32 {
        if coord < 0.0 {
            max - coord
        } else if coord > max {
            coord - max
        } else {
            coord
        }
    }
}

impl StellarObject for Vaisseau {
    /// Obtient la position actuelle de le vaisseau.
    ///
    /// # Retour
    ///
    /// La position sous forme de `Vec2`.
    fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Définit une nouvelle position pour le vaisseau.
    ///
    /// # Arguments
    ///
    /// * `new_position` - La nouvelle position sous forme de `Vec2`.
    fn set_position(&mut self, new_position: Vec2) {
        self.position = new_position;
    }

    /// Obtient la vitesse actuelle de le vaisseau.
    ///
    /// # Retour
    ///
    /// La vitesse sous forme de `Vec2`.
    fn get_speed(&self) -> Vec2 {
        self.speed
    }

    /// Définit une nouvelle vitesse pour le vaisseau.
    ///
    /// # Arguments
    ///
    /// * `new_speed` - La nouvelle vitesse sous forme de `Vec2`.
    fn set_speed(&mut self, new_speed: Vec2) {
        self.speed = new_speed;
    }

    /// Met à jour la position de le vaisseau en fonction de sa vitesse et de l'entrée du joueur.
    ///
    /// Les touches directionnelles (`Up`, `Down`, `Left`, `Right`) contrôlent la rotation et
    /// l'accélération de le vaisseau. Un effet de friction est appliqué pour ralentir naturellement le vaisseau.
    fn update_position(&mut self) {
        let mut acceleration = Vec2::ZERO;

        if is_key_down(KeyCode::Right) {
            self.rotation += 0.1;
        };

        if is_key_down(KeyCode::Left) {
            self.rotation -= 0.1;
        }

        if is_key_down(KeyCode::Up) {
            acceleration -= Vec2::new(self.rotation.sin(), self.rotation.cos());
        } else if is_key_down(KeyCode::Down) {
            acceleration += Vec2::new(self.rotation.sin(), self.rotation.cos());
        } else if self.speed.length() > 0.01 {
            self.speed *= 0.995; // Friction : ralentir progressivement
        } else {
            self.speed = Vec2::ZERO; // Vitesse très faible, donc arrêt complet
        }

        let new_speed = self.speed + acceleration;

        if new_speed.length() > 1. {
            self.set_speed(new_speed.normalize());
        } else {
            self.set_speed(new_speed);
        }

        let new_position = Self::bound_position(self.position + self.speed);
        self.set_position(new_position);
    }

    /// Gère une collision impliquant le vaisseau.
    ///
    /// Réduit les points de bouclier en fonction du niveau de l'astéroïde en collision.
    ///
    /// # Arguments
    ///
    /// * `asteroid_level` - Niveau de l'astéroïde (détermine les dégâts infligés).
    /// * `_` - Indique si la collision est confirmée (non utilisé ici).
    /// * `_` - Le vecteur de vitesse de l'objet en collision (non utilisé ici).
    ///
    /// # Retour
    ///
    /// Toujours `None`, car le vaisseau ne génère pas de nouveaux objets après une collision.
    fn handle_collision(
        &mut self,
        asteroid_level: u8,
        _: bool,
        _: Vec2,
    ) -> Option<(Asteroid, Asteroid)> {
        let dmg = match asteroid_level {
            1 => 1.,
            2 => 2.,
            3 => 3.,
            _ => 0.,
        };
        self.dmg_shield(dmg);
        None
    }
}
