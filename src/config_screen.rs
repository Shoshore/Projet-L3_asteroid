use macroquad::prelude::*;

/// Structure représentant l'écran de configuration du jeu.
pub struct ConfigScreen {
    asteroid_count: i32,         // Nombre d'astéroïdes
    asteroid_speed: f32,         // Vitesse des astéroïdes
    slider_width: f32,           // Largeur des sliders
    button_size: Vec2,           // Taille des boutons
    end_message: Option<String>, // Message de fin (optionnel)
}

impl ConfigScreen {
    /// Crée un nouvel écran de configuration avec les valeurs par défaut.
    ///
    /// # Retour
    ///
    /// Une instance de `ConfigScreen` initialisée avec des valeurs par défaut.
    pub fn new() -> Self {
        ConfigScreen {
            asteroid_count: 10,  // Valeur par défaut pour le nombre d'astéroïdes
            asteroid_speed: 1.0, // Valeur par défaut pour la vitesse des astéroïdes
            slider_width: 300.0, // Largeur par défaut des sliders
            button_size: Vec2::new(200.0, 50.0), // Taille par défaut des boutons
            end_message: None,   // Aucun message de fin par défaut
        }
    }

    /// Met à jour l'état de l'écran de configuration en fonction des interactions de l'utilisateur.
    pub fn update(&mut self) {
        // Définir la position du slider pour le nombre d'astéroïdes
        let asteroid_slider_x = screen_width() * 0.5 - self.slider_width / 2.0;
        let asteroid_slider_y = screen_height() * 0.4;

        // Dessiner la barre du slider pour le nombre d'astéroïdes
        let asteroid_value_x = asteroid_slider_x
            + ((self.asteroid_count as f32 - 1.) / (25. - 1.)) * self.slider_width;
        draw_line(
            asteroid_slider_x,
            asteroid_slider_y,
            asteroid_value_x,
            asteroid_slider_y,
            5.0,
            RED, // Partie rouge de la barre (complète)
        );
        draw_line(
            asteroid_value_x,
            asteroid_slider_y,
            asteroid_slider_x + self.slider_width,
            asteroid_slider_y,
            5.0,
            GREEN, // Partie verte de la barre (vide)
        );

        // Détecter si la souris est au-dessus du slider pour le nombre d'astéroïdes
        let mouse_pos = mouse_position();
        let is_mouse_on_asteroid_slider = mouse_pos.0 >= asteroid_slider_x
            && mouse_pos.0 <= asteroid_slider_x + self.slider_width
            && (mouse_pos.1 - asteroid_slider_y).abs() <= 10.0;

        // Interaction avec le slider pour ajuster le nombre d'astéroïdes
        if is_mouse_button_down(MouseButton::Left) && is_mouse_on_asteroid_slider {
            let mouse_x = mouse_position().0;
            if mouse_x >= asteroid_slider_x && mouse_x <= asteroid_slider_x + self.slider_width {
                self.asteroid_count =
                    ((mouse_x - asteroid_slider_x) / self.slider_width * (25. - 1.) + 1.) as i32;
                self.asteroid_count = self.asteroid_count.clamp(1, 25); // Limiter à 1-25 astéroïdes
            }
        }

        // Définir la position du slider pour la vitesse des astéroïdes
        let speed_slider_x = screen_width() * 0.5 - self.slider_width / 2.0;
        let speed_slider_y = screen_height() * 0.5 + 50.0; // Position un peu plus bas

        // Dessiner la barre du slider pour la vitesse des astéroïdes
        let speed_value_x =
            speed_slider_x + ((self.asteroid_speed - 0.3) / (5.0 - 0.3)) * self.slider_width;
        draw_line(
            speed_slider_x,
            speed_slider_y,
            speed_value_x,
            speed_slider_y,
            5.0,
            RED, // Partie rouge de la barre (complète)
        );
        draw_line(
            speed_value_x,
            speed_slider_y,
            speed_slider_x + self.slider_width,
            speed_slider_y,
            5.0,
            GREEN, // Partie verte de la barre (vide)
        );

        // Détecter si la souris est au-dessus du slider pour la vitesse des astéroïdes
        let is_mouse_on_speed_slider = mouse_pos.0 >= speed_slider_x
            && mouse_pos.0 <= speed_slider_x + self.slider_width
            && (mouse_pos.1 - speed_slider_y).abs() <= 10.0;

        // Interaction avec le slider pour ajuster la vitesse des astéroïdes
        if is_mouse_button_down(MouseButton::Left) && is_mouse_on_speed_slider {
            let mouse_x = mouse_position().0;
            if mouse_x >= speed_slider_x && mouse_x <= speed_slider_x + self.slider_width {
                self.asteroid_speed = ((mouse_x - speed_slider_x) / self.slider_width * (5. - 0.3)
                    + 0.3)
                    .clamp(0.3, 5.0); // Limiter la vitesse entre 0.3 et 5.0
            }
        }
    }

    /// Dessine l'écran de configuration avec tous les éléments graphiques.
    pub fn draw(&self) {
        clear_background(BLACK); // Fond noir pour l'écran

        // Afficher le texte pour le nombre d'astéroïdes
        draw_text(
            "Choisissez le nombre d'astéroïdes :",
            screen_width() * 0.5 - 200.0,
            screen_height() * 0.3,
            30.0,
            WHITE, // Texte en blanc
        );

        // Afficher la valeur actuelle du nombre d'astéroïdes
        draw_text(
            &format!("Astéroïdes : {}", self.asteroid_count),
            screen_width() * 0.5 - 50.0,
            screen_height() * 0.5,
            30.0,
            WHITE,
        );

        // Afficher la valeur actuelle de la vitesse des astéroïdes
        draw_text(
            &format!("Vitesse des astéroïdes : {:.1}", self.asteroid_speed),
            screen_width() * 0.5 - 100.0,
            screen_height() * 0.55,
            30.0,
            WHITE,
        );

        // Dessiner le bouton "Commencer"
        let button_position = Vec2::new(
            screen_width() * 0.5 - self.button_size.x / 2.0,
            screen_height() * 0.6,
        );
        draw_rectangle(
            button_position.x,
            button_position.y,
            self.button_size.x,
            self.button_size.y,
            GRAY, // Bouton gris
        );
        draw_text(
            "Commencer",
            button_position.x + 50.0,
            button_position.y + 30.0,
            25.0,
            WHITE, // Texte en blanc
        );

        // Dessiner le bouton "Exit"
        let exit_button_position = Vec2::new(
            button_position.x,
            button_position.y + self.button_size.y + 10.0,
        ); // Espacement entre les deux boutons
        draw_rectangle(
            exit_button_position.x,
            exit_button_position.y,
            self.button_size.x,
            self.button_size.y,
            RED, // Bouton rouge
        );
        draw_text(
            "Exit",
            exit_button_position.x + 70.0,
            exit_button_position.y + 30.0,
            25.0,
            WHITE, // Texte en blanc
        );

        // Afficher le message de fin si défini
        if let Some(ref message) = self.end_message {
            draw_text(
                message,
                screen_width() * 0.5 - message.len() as f32 * 5.0,
                screen_height() * 0.2,
                30.0,
                YELLOW, // Texte en jaune
            );
        }
    }

    /// Vérifie si le bouton "Commencer" a été pressé.
    ///
    /// # Retour
    ///
    /// `true` si le bouton a été pressé, sinon `false`.
    pub fn is_start_pressed(&self) -> bool {
        let button_position = Vec2::new(
            screen_width() / 2. - self.button_size.x / 2.0,
            screen_height() * 0.6,
        );
        let mouse = mouse_position();

        is_mouse_button_pressed(MouseButton::Left)
            && mouse.0 > button_position.x
            && mouse.0 < button_position.x + self.button_size.x
            && mouse.1 > button_position.y
            && mouse.1 < button_position.y + self.button_size.y
    }

    /// Vérifie si le bouton "Exit" a été pressé.
    ///
    /// # Retour
    ///
    /// `true` si le bouton a été pressé, sinon `false`.
    pub fn is_exit_pressed(&self) -> bool {
        let mouse = mouse_position();
        let exit_button_position = Vec2::new(
            screen_width() * 0.5 - self.button_size.x / 2.0,
            screen_height() * 0.6 + self.button_size.y + 10.0,
        );

        is_mouse_button_pressed(MouseButton::Left)
            && mouse.0 > exit_button_position.x
            && mouse.0 < exit_button_position.x + self.button_size.x
            && mouse.1 > exit_button_position.y
            && mouse.1 < exit_button_position.y + self.button_size.y
    }

    /// Retourne le nombre actuel d'astéroïdes.
    ///
    /// # Retour
    ///
    /// Le nombre d'astéroïdes sous forme d'un entier.
    pub fn get_asteroid_count(&self) -> i32 {
        self.asteroid_count
    }

    /// Retourne la vitesse actuelle des astéroïdes.
    ///
    /// # Retour
    ///
    /// La vitesse des astéroïdes sous forme de flottant.
    pub fn get_asteroid_speed(&self) -> f32 {
        self.asteroid_speed
    }

    /// Définit le message de fin à afficher.
    ///
    /// # Paramètres
    ///
    /// * `message` - Le message de fin à afficher.
    pub fn set_end_message(&mut self, message: &str) {
        self.end_message = Some(message.to_string());
    }
}
