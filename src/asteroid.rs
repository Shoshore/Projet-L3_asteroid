use crate::stellarobject::StellarObject;
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

/// Représente un astéroïde dans le jeu.
/// Les astéroïdes se déplacent, peuvent entrer en collision avec d'autres objets,
/// et se divisent en deux astéroïdes plus petits lors d'une collision avec un missile
/// si leur niveau est supérieur à 1.
pub struct Asteroid {
    /// Position actuelle de l'astéroïde dans l'espace de jeu.
    position: Vec2,
    /// Vecteur de vitesse de l'astéroïde.
    speed: Vec2,
    /// Niveau de l'astéroïde (3 = grand, 2 = moyen, 1 = petit).
    level: u8,
    /// Indique si l'astéroïde a été impliqué dans une collision.
    has_collided: bool,
}

impl Asteroid {
    /// Crée une nouvelle instance d'`Asteroid`.
    ///
    /// L'astéroïde est généré avec une position aléatoire autour des bords de l'écran.
    ///
    /// # Arguments
    ///
    /// * `level` - Niveau de l'astéroïde (taille).
    /// * `speed` - Vecteur de vitesse initiale.
    /// * `level_size` - Tuple représentant les tailles des astéroïdes pour les niveaux 3, 2 et 1.
    ///
    /// # Retour
    ///
    /// Une instance d'`Asteroid`.
    pub fn new(
        level: u8,
        speed: Vec2,
        level_size: (f32, f32, f32),
        position: Option<Vec2>,
    ) -> Self {
        Self {
            position: position.unwrap_or_else(|| Self::random_position(level, level_size)),
            speed,
            level,
            has_collided: false,
        }
    }

    /// Retourne le niveau actuel de l'astéroïde.
    ///
    /// # Retour
    ///
    /// Niveau de l'astéroïde.
    pub fn get_level(&self) -> u8 {
        self.level
    }

    /// Indique si l'astéroïde a été impliqué dans une collision.
    ///
    /// # Retour
    ///
    /// `true` si l'astéroïde a été en collision, sinon `false`.
    pub fn get_collided(&self) -> bool {
        self.has_collided
    }

    /// Génère une position aléatoire autour des bords de l'écran pour un nouvel astéroïde.
    ///
    /// # Arguments
    ///
    /// * `level` - Niveau de l'astéroïde, utilisé pour déterminer sa taille.
    /// * `level_size` - Tuple des tailles des astéroïdes pour les niveaux 3, 2 et 1.
    ///
    /// # Retour
    ///
    /// Une position `Vec2` autour des bords de l'écran.
    fn random_position(level: u8, level_size: (f32, f32, f32)) -> Vec2 {
        let mut rng = thread_rng();
        let size = match level {
            3 => level_size.0,
            2 => level_size.1,
            1 => level_size.2,
            _ => 0.0,
        };
        let side = rng.gen_range(0..4);
        match side {
            0 => vec2(rng.gen_range(0.0..screen_width()), -size),
            1 => vec2(screen_width() + size, rng.gen_range(0.0..screen_height())),
            2 => vec2(rng.gen_range(0.0..screen_width()), screen_height() + size),
            3 => vec2(-size, rng.gen_range(0.0..screen_height())),
            _ => unreachable!(),
        }
    }

    /// Contraint la position de l'astéroïde à rester dans les limites de l'écran.
    ///
    /// Si la position dépasse les limites, elle est ramenée de l'autre côté de l'écran.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position actuelle de l'astéroïde.
    ///
    /// # Retour
    ///
    /// Une nouvelle position contrainte à l'écran.
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
            max + coord
        } else if coord > max {
            coord - max
        } else {
            coord
        }
    }

    /// Divise un astéroïde en deux plus petits lors d'une collision avec un missile.
    ///
    /// Les deux nouveaux astéroïdes se déplacent dans des directions perpendiculaires
    /// à la direction du missile qui a causé la collision.
    ///
    /// # Arguments
    ///
    /// * `speed_missile` - Vecteur de vitesse du missile.
    ///
    /// # Retour
    ///
    /// Un tuple contenant deux nouveaux astéroïdes.
    pub fn split_asteroid(&mut self, speed_missile: Vec2) -> (Asteroid, Asteroid) {
        let asteroid_speed_norm = self.speed.length();

        // Calcul de deux directions perpendiculaires au vecteur du missile
        let perpendicular_direction_1 = Vec2::new(-speed_missile.y, speed_missile.x).normalize();
        let perpendicular_direction_2 = Vec2::new(speed_missile.y, -speed_missile.x).normalize();

        (
            Asteroid {
                position: self.position,
                speed: perpendicular_direction_1 * asteroid_speed_norm,
                level: self.level - 1,
                has_collided: false,
            },
            Asteroid {
                position: self.position,
                speed: perpendicular_direction_2 * asteroid_speed_norm,
                level: self.level - 1,
                has_collided: false,
            },
        )
    }
}

impl StellarObject for Asteroid {
    /// Obtient la position actuelle de l'astéroïde.
    ///
    /// # Retour
    ///
    /// La position sous forme de `Vec2`.
    fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Définit une nouvelle position pour l'astéroïde.
    ///
    /// # Arguments
    ///
    /// * `new_position` - La nouvelle position sous forme de `Vec2`.
    fn set_position(&mut self, new_position: Vec2) {
        self.position = new_position;
    }

    /// Obtient la vitesse actuelle de l'astéroïde.
    ///
    /// # Retour
    ///
    /// La vitesse sous forme de `Vec2`.
    fn get_speed(&self) -> Vec2 {
        self.speed
    }

    /// Définit une nouvelle vitesse pour l'astéroïde.
    ///
    /// # Arguments
    ///
    /// * `new_speed` - La nouvelle vitesse sous forme de `Vec2`.
    fn set_speed(&mut self, new_speed: Vec2) {
        self.speed = new_speed;
    }

    /// Met à jour la position de l'astéroïde en fonction de sa vitesse.
    ///
    /// La position est contrainte aux limites de l'écran grâce à l'effet "wrap-around".
    fn update_position(&mut self) {
        self.position += self.speed;
        self.position = Self::bound_position(self.position);
    }

    /// Gère une collision impliquant l'astéroïde.
    ///
    /// Si l'astéroïde est en collision avec un missile et qu'il est de niveau supérieur à 1,
    /// il est divisé en deux plus petits.
    ///
    /// # Arguments
    ///
    /// * `object` - Type de l'objet impliqué dans la collision (1 = missile).
    /// * `collided` - Indique si la collision est confirmée.
    /// * `speed_missile` - Vecteur de vitesse du missile.
    ///
    /// # Retour
    ///
    /// Un tuple contenant deux nouveaux astéroïdes si une division a lieu, sinon `None`.
    fn handle_collision(
        &mut self,
        object: u8,
        collided: bool,
        speed_missile: Vec2,
    ) -> Option<(Asteroid, Asteroid)> {
        self.has_collided = collided;
        if self.has_collided && (object == 1) && (self.level > 1) {
            return Some(self.split_asteroid(speed_missile));
        }
        None
    }
}
