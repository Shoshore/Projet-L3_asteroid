use ::rand::{thread_rng, Rng};
use macroquad::audio::{load_sound, play_sound, stop_sound, PlaySoundParams, Sound};
use macroquad::prelude::*;
use std::f32::consts::PI;

use asteroid::Asteroid;
use config_screen::ConfigScreen;
use missile::Missile;
use stellarobject::StellarObject;
use vaisseau::Vaisseau;

mod asteroid;
mod config_screen;
mod missile;
mod stellarobject;
mod vaisseau;

/// Dessine une texture centrée sur une position donnée avec une taille spécifique et une rotation.
///
/// Cette fonction utilise la méthode `draw_texture_ex` pour afficher une texture à l'écran,
/// en ajustant sa position pour qu'elle soit centrée sur le point spécifié et en appliquant
/// une taille et une rotation personnalisées.
///
/// # Paramètres
///
/// - `texture` :  
///   Une référence à l'objet `Texture2D` représentant la texture à dessiner.  
///   Cette texture sera affichée à la position spécifiée avec les transformations appliquées.
///
/// - `position` :  
///   Un objet `Vec2` définissant les coordonnées `(x, y)` du centre de la texture sur l'écran.  
///   La texture sera centrée autour de ce point.
///
/// - `size` :  
///   Un `f32` indiquant la taille utilisée pour redimensionner la texture. La texture est dessinée
///   avec une largeur et une hauteur proportionnelles à `size * 1.9`.
///
/// - `rotation` :  
///   Un `f32` représentant l'angle de rotation (en radians) appliqué à la texture.  
///   La rotation se fait autour du centre de la texture.
///
/// # Détails
///
/// - La fonction ajuste automatiquement la position pour compenser l'origine de la texture,
///   qui est initialement le coin supérieur gauche.
/// - La taille finale de la texture à l'écran est proportionnelle à `size * 1.9`, ce qui permet
///   de la redimensionner dynamiquement tout en respectant ses proportions initiales.
/// - La rotation est appliquée autour du centre visuel de la texture.
fn draw_centered_texture(texture: &Texture2D, position: Vec2, size: f32, rotation: f32) {
    draw_texture_ex(
        texture,
        position.x - (size),
        position.y - (size),
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(size * 1.9, size * 1.9)),
            rotation,
            ..Default::default()
        },
    );
}

/// Dessine des astéroïdes groupés par niveau en utilisant des sprites différents pour chaque niveau.
///
/// Cette fonction regroupe les astéroïdes par leur niveau (1, 2, ou 3) et dessine chaque groupe
/// avec un sprite spécifique. Les astéroïdes sont redimensionnés en fonction de leur niveau,
/// et leur position est ajustée pour qu'ils soient centrés lors du dessin.
///
/// # Paramètres
///
/// - `asteroids` :  
///   Une tranche mutable de `Asteroid`. Chaque astéroïde de cette tranche est lu pour obtenir
///   sa position, son niveau, et sa taille calculée.
///
/// - `level_size` :  
///   Un tuple `(f32, f32, f32)` définissant la taille associée à chaque niveau d'astéroïde.  
///   Par exemple, `level_size.0` correspond à la taille des astéroïdes de niveau 3.
///
/// - `sprites` :  
///   Un tableau de trois références vers des objets `Texture2D`, où chaque sprite représente
///   visuellement un niveau d'astéroïde.  
///   Par convention :  
///   - `sprites[0]` est utilisé pour les astéroïdes de niveau 1.  
///   - `sprites[1]` pour les astéroïdes de niveau 2.  
///   - `sprites[2]` pour les astéroïdes de niveau 3.
///
/// # Fonctionnement
///
/// 1. Les astéroïdes sont regroupés par niveau à l'aide d'un tableau `batched_draws`,
///    où chaque groupe contient les positions et tailles des astéroïdes du même niveau.
/// 2. Une boucle sur les groupes dessine chaque astéroïde à l'aide du sprite correspondant.
/// 3. La texture de chaque astéroïde est centrée sur sa position et redimensionnée
///    proportionnellement à sa taille (dépendant de son niveau).
fn draw_asteroids_batched(
    asteroids: &mut [Asteroid],
    level_size: (f32, f32, f32),
    sprites: [&Texture2D; 3],
) {
    // Prépare une liste de dessins pour chaque niveau d'astéroïde
    let mut batched_draws: [Vec<(Vec2, f32)>; 3] = [Vec::new(), Vec::new(), Vec::new()];

    // Grouper les astéroïdes par leur niveau
    for asteroid in asteroids.iter_mut() {
        let level = (asteroid.get_level() - 1) as usize;
        let size = asteroid_level(asteroid, level_size);
        let position = asteroid.get_position();
        batched_draws[level].push((position, size));
    }

    // Dessiner les astéroïdes groupés par niveau
    for (level, draws) in batched_draws.iter().enumerate() {
        let sprite = sprites[level];
        for (position, size) in draws {
            draw_texture_ex(
                sprite,
                position.x - size,
                position.y - size,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(size * 1.9, size * 1.9)),
                    rotation: 0.0,
                    ..Default::default()
                },
            );
        }
    }
}

/// Dessine l'arrière-plan de l'écran en utilisant une texture spécifiée.
///
/// Cette fonction dessine une texture couvrant toute la surface de l'écran. La texture
/// est redimensionnée pour correspondre à la taille actuelle de l'écran afin de couvrir
/// l'intégralité de la vue.
///
/// # Paramètres
///
/// - `sprite` :  
///   La texture à dessiner en arrière-plan. Cette texture est utilisée pour remplir toute
///   la fenêtre du jeu, sans tenir compte de la position ou de la rotation.
///
/// # Remarques
///
/// - La texture est étendue pour couvrir toute la fenêtre de jeu, donc son ratio de dimensions
///   peut ne pas être conservé. Pour éviter cela, la texture devrait être carrée ou avoir un
///   rapport d'aspect similaire à la fenêtre du jeu.
fn draw_background(sprite: &Texture2D) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    draw_texture_ex(
        sprite,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(screen_width, screen_height)),
            ..Default::default()
        },
    );
}

/// Dessine le vaisseau à une position et une taille données.
///
/// Cette fonction dessine le vaisseau à sa position actuelle, avec la taille et la rotation spécifiées.
/// Elle utilise la fonction `draw_centered_texture` pour s'assurer que le vaisseau est correctement
/// centré à la position donnée. La taille de l'vaisseau est ajustée en fonction du paramètre `hauteur_vaisseau`
/// et la rotation est appliquée à l'objet pour lui donner l'orientation correcte.
///
/// # Paramètres
///
/// - `vaisseau` :  
///   L'objet `vaisseau` à dessiner. La fonction utilise sa position et sa rotation actuelles.
///
/// - `sprite` :  
///   La texture à utiliser pour dessiner le vaisseau.
///
/// - `hauteur_vaisseau` :  
///   La hauteur de le vaisseau, qui détermine sa taille à l'écran. Cette hauteur est utilisée
///   pour ajuster l'échelle de la texture de le vaisseau.
///
/// # Remarques
///
/// - Cette fonction utilise la rotation de le vaisseau, donc il est important que l'objet `Vaisseau`
///   ait une valeur de rotation mise à jour pour que l'affichage soit correct.
fn draw_vaisseau(vaisseau: &Vaisseau, sprite: &Texture2D, hauteur_vaisseau: f32) {
    draw_centered_texture(
        sprite,
        vaisseau.get_position(),
        hauteur_vaisseau,
        vaisseau.get_rotation(),
    );
}

/// Dessine les missiles à leur position respective en utilisant une texture spécifiée.
///
/// Cette fonction dessine chaque missile sur l'écran à sa position respective. Les missiles
/// sont dessinés en utilisant la texture fournie et leur rayon détermine leur taille à l'écran.
///
/// # Paramètres
///
/// - `missiles` :  
///   Une référence mutable à une liste de `Missile`. Chaque missile est dessiné à sa position
///   actuelle avec un rayon ajusté.
///
/// - `rayon_missile` :  
///   Le rayon du missile, utilisé pour déterminer sa taille lorsqu'il est dessiné.
///
/// - `sprite` :  
///   La texture à utiliser pour dessiner les missiles.
///
/// # Remarques
///
/// - Les missiles sont dessinés à leur position actuelle, mais il est important de s'assurer
///   que leur position est mise à jour à chaque itération pour obtenir un mouvement fluide à l'écran.
/// - Le rayon du missile est utilisé pour calculer sa taille à l'écran, donc il peut être ajusté
///   dynamiquement selon les besoins du jeu.
fn draw_missiles(missiles: &Vec<Missile>, rayon_missile: f32, sprite: &Texture2D) {
    for missile in missiles {
        draw_centered_texture(sprite, missile.get_position(), rayon_missile, 0.);
    }
}

/// Met à jour la position de le vaisseau en fonction de sa vitesse actuelle.
/// Cette fonction utilise la méthode interne `update_position` de l'objet `Vaisseau`
/// pour recalculer sa position en fonction de sa vitesse et de la direction actuelle.
///
/// # Paramètres
///
/// - `vaisseau` :  
///   Une référence mutable à un objet `Vaisseau`. La position de cet vaisseau est mise à jour
///   directement en fonction de sa vitesse et de son orientation.
///
/// # Fonctionnement
///
/// - La méthode `update_position` de l'vaisseau est appelée, ce qui met à jour sa position.
/// - Les limites de l'écran ou d'autres contraintes ne sont pas gérées ici ; il est supposé
///   que cela est pris en charge par d'autres parties du code.
fn update_model_vaisseau(vaisseau: &mut Vaisseau) {
    vaisseau.update_position();
}

/// Met à jour la position de chaque astéroïde de la liste.
/// Chaque astéroïde voit sa position recalculée en fonction de sa vitesse actuelle
/// et de sa direction, via la méthode interne `update_position`.
///
/// # Paramètres
///
/// - `asteroids` :  
///   Une référence mutable à un vecteur d'objets `Asteroid`. La position de chaque astéroïde
///   est mise à jour individuellement.
///
/// # Fonctionnement
///
/// - La méthode `update_position` de chaque astéroïde est appelée.
/// - Cette fonction ne vérifie pas les collisions ni les limites de l'écran ; ces aspects doivent
///   être gérés ailleurs.
fn update_asteroids(asteroids: &mut Vec<Asteroid>) {
    for asteroid in asteroids {
        asteroid.update_position();
    }
}

/// Met à jour la position de chaque missile et retire ceux qui sont sortis de l'écran.
/// La position de chaque missile est recalculée en fonction de sa vitesse, et les missiles
/// qui sortent des limites de l'écran sont supprimés de la liste.
///
/// # Paramètres
///
/// - `missiles` :  
///   Une référence mutable à un vecteur d'objets `Missile`. Chaque missile voit sa position
///   mise à jour, et ceux qui sont hors de l'écran sont supprimés.
///
/// # Fonctionnement
///
/// 1. La méthode `update_position` de chaque missile est appelée pour recalculer sa position.
/// 2. Les missiles qui dépassent les limites de l'écran sont identifiés à l'aide de la méthode
///    `is_off_screen`.  
/// 3. Ces missiles sont ensuite retirés de la liste à l'aide de `retain`.
fn update_missiles(missiles: &mut Vec<Missile>) {
    for missile in missiles.iter_mut() {
        missile.update_position();
    }
    missiles.retain(|missile| !missile.is_off_screen(screen_width(), screen_height()));
}

/// Vérifie et gère les collisions entre le vaisseau et les astéroïdes.
/// Cette fonction détecte les collisions entre le vaisseau et les astéroïdes, applique une force
/// gravitationnelle si le vaisseau est à proximité, et gère les impacts directs en mettant à jour
/// les états de le vaisseau et des astéroïdes.
///
/// # Paramètres
///
/// - `vaisseau` :  
///   Référence mutable à un objet `Vaisseau`. La fonction vérifie si cet vaisseau entre en collision
///   avec des astéroïdes et met à jour sa position, sa vitesse, et son état en conséquence.
/// - `asteroids` :  
///   Slice mutable d'objets `Asteroid` représentant les astéroïdes présents dans le jeu.
///   Chaque astéroïde peut influencer le vaisseau via la gravité ou entrer en collision avec lui.
/// - `hauteur_vaisseau` :  
///   Un `f32` représentant la hauteur de le vaisseau, utilisée pour calculer son rayon de collision.
/// - `level_size` :  
///   Tuple `(f32, f32, f32)` définissant les tailles des astéroïdes en fonction de leur niveau.
/// - `gravite_dist` :  
///   Un `f32` définissant la portée maximale de la gravité exercée par les astéroïdes.
///   Si le vaisseau est dans cette portée, une force gravitationnelle est appliquée.
/// - `ship_hit` :  
///   Une `Option<&Sound>` représentant le son à jouer lorsqu'une collision directe se produit
///   entre le vaisseau et un astéroïde. Si `None`, aucun son ne sera joué.
///
/// # Fonctionnement
///
/// 1. **Calcul de la distance :**  
///    Pour chaque astéroïde, la distance entre le vaisseau et l'astéroïde est calculée.
/// 2. **Force gravitationnelle :**  
///    Si le vaisseau est à une distance inférieure à `gravite_dist + taille_astéroïde`, une force
///    gravitationnelle est appliquée à le vaisseau. Cette force est calculée avec la fonction
///    `calculate_gravity`.
/// 3. **Collision directe :**  
///    Si la distance entre le vaisseau et un astéroïde est inférieure à la somme de leurs rayons
///    (collision), alors :
///    - Le son spécifié dans `ship_hit` est joué (si fourni).
///    - L'état de le vaisseau est mis à jour avec la méthode `handle_collision`.
///    - L'astéroïde est marqué comme "collidé" et son état est mis à jour.
fn check_vaisseau_asteroids(
    vaisseau: &mut Vaisseau,
    asteroids: &mut [Asteroid],
    hauteur_vaisseau: f32,
    level_size: (f32, f32, f32),
    gravite_dist: f32,
    ship_hit: Option<&Sound>,
) {
    let vaisseau_position = vaisseau.get_position();
    let vaisseau_radius = hauteur_vaisseau;

    for asteroid in asteroids.iter_mut() {
        let asteroid_size = asteroid_level(asteroid, level_size);
        let distance_squared = (asteroid.get_position() - vaisseau_position).length_squared();
        let collision_distance_squared = (asteroid_size + vaisseau_radius).powi(2);
        if distance_squared > collision_distance_squared {
            continue;
        }
        let dist_gravity = asteroid_size + gravite_dist;
        if distance_squared <= dist_gravity.powi(2) {
            let vitesse = calculate_gravity(vaisseau, asteroid, 0.5, &hauteur_vaisseau, level_size);
            vaisseau.set_speed(vitesse);
        }

        if distance_squared <= collision_distance_squared && !asteroid.get_collided() {
            if let Some(sound) = ship_hit {
                play_game_sound(sound, false, 0.1);
            }

            vaisseau.handle_collision(asteroid.get_level(), true, Vec2::ZERO);
            asteroid.handle_collision(0, true, Vec2::ZERO);
        }
    }
}

/// Calcule la force gravitationnelle exercée par un astéroïde sur le vaisseau.
/// Cette force est calculée en fonction des positions respectives de le vaisseau et de l'astéroïde,
/// ainsi que de paramètres comme la constante gravitationnelle, la taille de le vaisseau,
/// et la taille de l'astéroïde.
///
/// # Paramètres
///
/// - `vaisseau` :
///   Référence à un objet `Vaisseau` représentant le vaisseau sur lequel la force gravitationnelle agit.
///   La position de le vaisseau est utilisée pour calculer la direction de la force.
/// - `asteroid` :
///   Référence mutable à un objet `Asteroid` représentant l'astéroïde qui exerce la force.
///   Sa position et sa taille (en fonction de son niveau) sont utilisées dans le calcul.
/// - `g_constant` :
///   Un `f32` représentant la constante gravitationnelle utilisée pour ajuster l'intensité
///   de la force. Une valeur plus élevée entraîne une force plus importante.
/// - `hauteur_vaisseau` :
///   Une référence à un `f32` représentant la hauteur de le vaisseau Utilisée pour calculer
///   la force en fonction de la masse supposée de le vaisseau.
/// - `level_size` :
///   Tuple `(f32, f32, f32)` représentant les tailles des niveaux d'astéroïdes. La taille de
///   l'astéroïde est déterminée en fonction de son niveau grâce à ce paramètre.
///
/// # Retour
///
/// Retourne un `Vec2` représentant la force gravitationnelle sous forme d'un vecteur :
/// - La direction du vecteur pointe de le vaisseau vers l'astéroïde.
/// - La magnitude est limitée par une valeur maximale pour éviter des forces excessives
///   qui pourraient déséquilibrer le jeu.
///
/// Si le vaisseau et l'astéroïde occupent exactement la même position, la fonction retourne
/// `Vec2::ZERO` pour éviter une division par zéro.
///
/// # Fonctionnement
///
/// 1. La direction de la force est calculée comme un vecteur normalisé allant de le vaisseau
///    vers l'astéroïde.
/// 2. La magnitude de la force est calculée à l'aide de la formule :
///    ```text
///    Force = (G * Taille_Vaisseau * Taille_Asteroide) / Distance²
///    ```
///    où :
///    - `G` est la constante gravitationnelle (`g_constant`).
///    - `Taille_Vaisseau` est donnée par `hauteur_vaisseau`.
///    - `Taille_Asteroide` est obtenue via la fonction `asteroid_level`.
/// 3. La magnitude est limitée à une valeur maximale (par défaut, 2.0) pour éviter des forces
///    gravitationnelles irréalistes.
/// 4. La force finale est calculée en combinant la direction et la magnitude.
fn calculate_gravity(
    vaisseau: &Vaisseau,
    asteroid: &mut Asteroid,
    g_constant: f32,
    hauteur_vaisseau: &f32,
    level_size: (f32, f32, f32),
) -> Vec2 {
    let vaisseau_pos = vaisseau.get_position();
    let asteroid_pos = asteroid.get_position();

    let direction = asteroid_pos - vaisseau_pos;
    let distance = direction.length();

    if distance == 0.0 {
        return Vec2::ZERO;
    }

    let unit_direction = direction / distance;

    let size_asteroid = asteroid_level(asteroid, level_size);

    let mut force_magnitude = g_constant * hauteur_vaisseau * size_asteroid / (distance * distance);

    let max = 2.;
    if force_magnitude > max {
        force_magnitude = max;
    }

    unit_direction * force_magnitude
}

/// Vérifie et gère les collisions entre les missiles et les astéroïdes dans le jeu.
/// Cette fonction détecte les collisions, met à jour les états des objets (missiles et astéroïdes),
/// joue un son lorsqu'un astéroïde est touché, met à jour le score et gère la division
/// des astéroïdes en morceaux si applicable.
///
/// # Paramètres
///
/// - `missiles` :
///   Référence mutable à un `Vec<Missile>` contenant la liste des missiles actifs dans le jeu.
///   Les missiles qui entrent en collision avec des astéroïdes seront marqués comme "collidés"
///   et supprimés de cette liste.
/// - `asteroids` :
///   Référence mutable à un `Vec<Asteroid>` contenant la liste des astéroïdes présents.
///   Les astéroïdes touchés par des missiles seront supprimés, et, s'ils peuvent se diviser,
///   les nouveaux fragments seront ajoutés à cette liste.
/// - `rayon_missile` :
///   Un `f32` représentant le rayon des missiles, utilisé pour calculer la distance nécessaire
///   pour qu'une collision soit détectée.
/// - `level_size` :
///   Tuple `(f32, f32, f32)` représentant les tailles des niveaux, utilisé pour déterminer la
///   taille des astéroïdes en fonction de leur niveau.
/// - `asteroid_hit` :
///   Une `Option<&Sound>` représentant le son à jouer lorsqu'un astéroïde est touché.
///   Si `None`, aucun son ne sera joué.
/// - `score` :
///   Référence mutable à un `i32` représentant le score du joueur. Le score sera augmenté
///   en fonction du niveau de l'astéroïde détruit (par exemple, un astéroïde de niveau 1 donne
///   10 points, un niveau 2 donne 20 points, etc.).
///
/// # Fonctionnement
///
/// 1. La fonction parcourt chaque missile et vérifie les collisions avec tous les astéroïdes.
/// 2. Une collision est détectée si la distance au carré entre le missile et l'astéroïde est
///    inférieure ou égale au carré de la somme de leurs rayons (`rayon_missile + taille_asteroid`).
/// 3. En cas de collision :
///    - Le son spécifié (`asteroid_hit`) est joué si fourni.
///    - Le score est mis à jour en fonction du niveau de l'astéroïde touché.
///    - Le missile est marqué comme "collidé".
///    - L'astéroïde est supprimé. S'il peut se diviser, deux nouveaux astéroïdes sont générés
///      avec des propriétés issues de la collision.
/// 4. Une fois toutes les collisions vérifiées, les missiles "collidés" sont supprimés, et les
///    nouveaux fragments d'astéroïdes sont ajoutés à la liste.
fn check_missiles_asteroids(
    missiles: &mut Vec<Missile>,
    asteroids: &mut Vec<Asteroid>,
    rayon_missile: f32,
    level_size: (f32, f32, f32),
    asteroid_hit: Option<&Sound>,
    score: &mut i32,
) {
    let mut asteroids_to_remove = Vec::new();
    let mut new_asteroids = Vec::new();

    for missile in missiles.iter_mut() {
        for (asteroid_index, asteroid) in asteroids.iter_mut().enumerate() {
            let distance_squared =
                (missile.get_position() - asteroid.get_position()).length_squared();
            let asteroid_size = asteroid_level(asteroid, level_size);
            let collision_distance_squared = (asteroid_size + rayon_missile).powi(2);
            if distance_squared >= collision_distance_squared {
                continue;
            }
            if let Some(sound) = asteroid_hit {
                play_game_sound(sound, false, 0.1);
            }

            *score += asteroid.get_level() as i32 * 10;
            missile.handle_collision(0, true, Vec2::ZERO);

            if let Some((asteroid_1, asteroid_2)) =
                asteroid.handle_collision(1, true, missile.get_speed())
            {
                new_asteroids.push(asteroid_1);
                new_asteroids.push(asteroid_2);
            }

            asteroids_to_remove.push(asteroid_index);
            break;
        }
    }

    missiles.retain(|missile| !missile.get_collided());

    asteroids_to_remove.sort_unstable_by(|a, b| b.cmp(a));
    for index in asteroids_to_remove {
        asteroids.remove(index);
    }

    asteroids.extend(new_asteroids);
}

/// Réinitialise l'état du jeu, en créant de nouveaux astéroïdes et réinitialisant le vaisseau, les missiles et le score.
/// Cette fonction prépare le jeu pour un nouveau départ en effaçant les objets existants et en initialisant de nouvelles instances.
///
/// # Paramètres
///
/// - `liste_asteroid` :
///   Référence mutable à un `Vec<Asteroid>` contenant la liste des astéroïdes du jeu. Cette liste
///   sera vidée et remplie avec de nouveaux astéroïdes générés.
/// - `vaisseau` :
///   Référence mutable à l'objet `Vaisseau`, qui sera réinitialisé avec des paramètres par défaut
///   ou des valeurs spéciales si le mode de test est activé.
/// - `missiles` :
///   Référence mutable à un `Vec<Missile>` contenant les missiles actifs. Cette liste sera vidée
///   pour un redémarrage propre.
/// - `level_size` :
///   Tuple `(f32, f32, f32)` représentant la taille des niveaux, utilisé pour déterminer les
///   positions et les limites des astéroïdes.
/// - `number_asteroid` :
///   Un `i32` représentant le nombre d'astéroïdes à générer.
/// - `asteroid_speed` :
///   Un `f32` représentant la vitesse des astéroïdes. Ce paramètre affecte les plages minimale
///   et maximale des vitesses des astéroïdes générés.
/// - `score` :
///   Référence mutable à un `i32` représentant le score du joueur, qui sera réinitialisé à 0.
/// - `test` :
///   Booléen indiquant si la fonction est appelée dans un contexte de test. Si `true`, des valeurs
///   par défaut simplifiées seront utilisées pour la position initiale de le vaisseau et l'état du dernier
///   tir :
///   - La position de le vaisseau sera fixée à `(0., 0.)`.
///   - L'état du dernier tir sera fixé à `0.`
///
///  # Fonctionnement
///
/// 1. - Rétinialise toutes les variables utile au bon fonctionnement du jeu.
/// 2. - Créer une quantité "number_asteroid" d'asteroids ayant une vitesse "asteroid_speed"
///         on cacule un mimum et un maximum avec cette speed.
fn reset_game(
    liste_asteroid: &mut Vec<Asteroid>,
    vaisseau: &mut Vaisseau,
    missiles: &mut Vec<Missile>,
    level_size: (f32, f32, f32),
    number_asteroid: i32,
    asteroid_speed: f32,
    score: &mut i32,
    test: bool,
) {
    let mut position = None;
    let mut last_shot = None;
    if test {
        position = Some(Vec2::new(0., 0.));
        last_shot = Some(0.);
    }
    liste_asteroid.clear();
    *vaisseau = Vaisseau::new(position, last_shot);
    missiles.clear();
    *score = 0;

    // Définir une plage dynamique pour la vitesse des astéroïdes
    let min_speed = 0.2 + (1.0 - asteroid_speed) * 0.4; // La borne inférieure se réduit avec la vitesse
    let max_speed = asteroid_speed * 2.0; // La borne supérieure est multipliée par la vitesse

    // Générer les astéroïdes
    for _ in 0..number_asteroid {
        let angle = thread_rng().gen_range(0.0..(2.0 * PI));
        let speed_magnitude = thread_rng().gen_range(min_speed..max_speed);

        let speed = Vec2::new(
            speed_magnitude * angle.cos(), // Composante x
            speed_magnitude * angle.sin(), // Composante y
        );
        liste_asteroid.push(asteroid::Asteroid::new(3, speed, level_size, position));
    }
}

/// Retourne la taille d'un astéroïde en fonction de son niveau.
/// Cette fonction mappe le niveau de l'astéroïde à une taille spécifique issue des paramètres `level_size`.
///
/// # Paramètres
///
/// - `asteroid` :
///   Référence mutable à un `Asteroid` pour lequel on veut calculer la taille.
///   La taille est déterminée en fonction de son niveau (1, 2 ou 3).
/// - `level_size` :
///   Tuple `(f32, f32, f32)` représentant les tailles associées à chaque niveau d'astéroïde :
///   - `level_size.0` : Taille pour les astéroïdes de niveau 3 (les plus grands).
///   - `level_size.1` : Taille pour les astéroïdes de niveau 2.
///   - `level_size.2` : Taille pour les astéroïdes de niveau 1 (les plus petits).
///
/// # Retour
///
/// Un `f32` représentant la taille de l'astéroïde basée sur son niveau :
/// - Si le niveau est 1, retourne `level_size.2`.
/// - Si le niveau est 2, retourne `level_size.1`.
/// - Si le niveau est 3, retourne `level_size.0`.
/// - Si le niveau est invalide (ni 1, 2, ni 3), retourne `0.0`.
fn asteroid_level(asteroid: &mut Asteroid, level_size: (f32, f32, f32)) -> f32 {
    match asteroid.get_level() {
        1 => level_size.2,
        2 => level_size.1,
        3 => level_size.0,
        _ => 0.0,
    }
}

/// Met à jour les échelles de taille des objets du jeu en fonction des dimensions actuelles de l'écran.
/// Cette fonction ajuste proportionnellement les tailles des objets à l'écran en fonction du changement de taille d'écran
/// depuis le dernier calcul.
///
/// # Paramètres
///
/// - `hauteur_vaisseau` :
///   Référence mutable à un `f32` représentant la hauteur de le vaisseau. Sa valeur sera mise à jour
///   en fonction de l'échelle calculée.
/// - `rayon_missile` :
///   Référence mutable à un `f32` représentant le rayon des missiles. Cette valeur sera ajustée
///   proportionnellement à l'échelle calculée.
/// - `level_size` :
///   Référence mutable à un tuple `(f32, f32, f32)` représentant les dimensions des niveaux.
///   Les trois composantes seront ajustées proportionnellement à l'échelle calculée.
/// - `last_screen_size` :
///   Référence mutable à un tuple `(f32, f32)` représentant la taille de l'écran lors du dernier calcul.
///   Ce paramètre sera mis à jour pour refléter les nouvelles dimensions de l'écran.
/// - `gravite_dist` :
///   Référence mutable à un `f32` représentant la distance de gravité utilisée dans le jeu. Sa valeur
///   sera ajustée selon l'échelle calculée.
/// - `test` :
///   Booléen indiquant si la fonction est appelée dans un contexte de test. Si `true`, les dimensions
///   de l'écran (`current_width` et `current_height`) seront fixées à des valeurs par défaut
///   (500 pour la largeur et 400 pour la hauteur).
///
/// #Fonctionnement
///
/// 1. - calcule de du ratio nouvelle taille / ancienne taille de l'écran
/// 2. - ajusté les tailles de toutes les variables
fn update_scale(
    hauteur_vaisseau: &mut f32,
    rayon_missile: &mut f32,
    level_size: &mut (f32, f32, f32),
    last_screen_size: &mut (f32, f32),
    gravite_dist: &mut f32,
    test: bool,
) {
    let mut current_width = 500.;
    let mut current_height = 400.;
    if !test {
        current_width = screen_width();
        current_height = screen_height();
    }

    let width_scale = current_width / last_screen_size.0;
    let height_scale = current_height / last_screen_size.1;

    let scale_factor = (width_scale + height_scale) / 2.0;

    // Mise à jour des tailles des objets
    *hauteur_vaisseau *= scale_factor;
    *rayon_missile *= scale_factor;
    level_size.0 *= scale_factor;
    level_size.1 *= scale_factor;
    level_size.2 *= scale_factor;
    *gravite_dist *= scale_factor;

    *last_screen_size = (current_width, current_height);
}

/// Joue un son dans le jeu.
/// Cette fonction utilise un objet `Sound` pour jouer un effet sonore ou une musique,
/// avec des options pour la mise en boucle et le contrôle du volume.
///
/// # Paramètres
///
/// - `sound` :
///   Référence à un objet `Sound` représentant le son à jouer. Ce son doit être préalablement
///   chargé en mémoire avant l'appel à cette fonction.
/// - `looped` :
///   Un booléen indiquant si le son doit être joué en boucle :
///   - `true` : Le son sera joué en boucle indéfiniment (ou jusqu'à ce qu'il soit explicitement arrêté).
///   - `false` : Le son sera joué une seule fois.
/// - `volume` :
///   Un `f32` représentant le volume du son. La valeur doit être comprise entre 0.0 (silencieux)
///   et 1.0 (volume maximal).
fn play_game_sound(sound: &Sound, looped: bool, volume: f32) {
    play_sound(sound, PlaySoundParams { looped, volume });
}

/// Contient les textures du jeu pour les différents objets.
///
/// - `sprite_vaisseau`: Texture2D de le vaisseau.
/// - `sprite_asteroid_3`: Texture2D des astéroïdes de niveau 3.
/// - `sprite_asteroid_2`: Texture2D des astéroïdes de niveau 2.
/// - `sprite_asteroid_1`: Texture2D des astéroïdes de niveau 1.
/// - `sprite_background`: Texture2D de l'arrière-plan.
/// - `sprite_meteor`: Texture2D des missiles.
struct Textures {
    sprite_vaisseau: Texture2D,
    sprite_asteroid_3: Texture2D,
    sprite_asteroid_2: Texture2D,
    sprite_asteroid_1: Texture2D,
    sprite_background: Texture2D,
    sprite_meteor: Texture2D,
}

/// Contient les sons du jeu pour les différents événements.
///
/// - `shoot`: Son lorsque le vaisseau tire. (Sound)
/// - `asteroid_hit`: Son lorsqu'un astéroïde est touché. (Sound)
/// - `background_music`: Musique de fond. (Sound)
/// - `win`: Son lorsque le joueur gagne. (Sound)
/// - `lose`: Son lorsque le joueur perd. (Sound)
/// - `ship_hit`: Son lorsque le vaisseau est touché. (Sound)
struct Sounds {
    shoot: Sound,
    asteroid_hit: Sound,
    background_music: Sound,
    win: Sound,
    lose: Sound,
    ship_hit: Sound,
}

/// Fonction principale du jeu, exécutée dans la boucle principale.
/// Gère la configuration, la mise à jour des objets, les entrées utilisateur et l'affichage.
///
/// # Retour
///
/// Cette fonction ne retourne rien, elle tourne indéfiniment tant que le jeu est en cours.
#[macroquad::main("BasicShapes")]
async fn main() {
    let target_fps: f32 = 120.0; // Limite de FPS
    let frame_duration: f32 = 1.0 / target_fps; // Durée cible par frame en secondes

    // Chargement des textures
    let textures = Textures {
        sprite_vaisseau: load_texture("./sprite/vaisseau.png").await.unwrap(),
        sprite_asteroid_3: load_texture("./sprite/asteroid_3.png").await.unwrap(),
        sprite_asteroid_2: load_texture("./sprite/asteroid_2.png").await.unwrap(),
        sprite_asteroid_1: load_texture("./sprite/asteroid_1.png").await.unwrap(),
        sprite_background: load_texture("./sprite/background.png").await.unwrap(),
        sprite_meteor: load_texture("./sprite/missile.png").await.unwrap(),
    };

    // Chargement des sons
    let sounds = Sounds {
        shoot: load_sound("./audio/shoot.wav").await.unwrap(),
        asteroid_hit: load_sound("./audio/asteroid_hit.wav").await.unwrap(),
        background_music: load_sound("./audio/background_music.wav").await.unwrap(),
        win: load_sound("./audio/win.wav").await.unwrap(),
        lose: load_sound("./audio/lose.wav").await.unwrap(),
        ship_hit: load_sound("./audio/ship_hit.wav").await.unwrap(),
    };

    // Initialisation des variables du jeu
    let mut score: i32 = 0;
    let begin_time = get_time();

    let mut last_screen_size: (f32, f32) = (screen_width(), screen_height());
    let mut hauteur_vaisseau: f32 = 30.;
    let mut rayon_missile: f32 = 7.;
    let mut level_size: (f32, f32, f32) = (40., 20., 10.);
    let mut gravite_dist = 30.;

    let mut liste_asteroid = Vec::new();
    let mut vaisseau: Vaisseau = Vaisseau::new(None, None);
    let mut missiles = Vec::new();

    let mut config_screen = ConfigScreen::new();
    let mut in_configuration = true;

    loop {
        let start_time = get_time(); // Début de la frame actuelle

        // Si l'écran de configuration est actif
        if in_configuration {
            stop_sound(&sounds.background_music);
            config_screen.draw();
            config_screen.update();

            if config_screen.is_start_pressed() {
                in_configuration = false;
                reset_game(
                    &mut liste_asteroid,
                    &mut vaisseau,
                    &mut missiles,
                    level_size,
                    config_screen.get_asteroid_count(),
                    config_screen.get_asteroid_speed(),
                    &mut score,
                    false,
                );
                stop_sound(&sounds.lose);
                stop_sound(&sounds.win);
                play_game_sound(&sounds.background_music, true, 0.1);
            }
            if config_screen.is_exit_pressed() {
                break;
            }
            next_frame().await;
        } else {
            // Si le bouclier de le vaisseau est épuisé
            if vaisseau.get_shield() < 0. {
                config_screen.set_end_message(&format!("Défaite ! Score : {}", score));
                play_game_sound(&sounds.lose, false, 0.1);
                in_configuration = true;
            } else if liste_asteroid.is_empty() {
                // Le joueur a gagné
                let time_bonus = ((begin_time - get_time()) as f32).round() as i32;
                let shield_bonus = (vaisseau.get_shield() as i32) * 5;

                score += time_bonus + shield_bonus;
                config_screen.set_end_message(&format!("Victoire ! Score : {}", score));
                play_game_sound(&sounds.win, false, 0.1);
                in_configuration = true;
            } else {
                // Gérer les entrées et mettre à jour l'état du jeu
                if is_key_down(KeyCode::Escape) {
                    config_screen.set_end_message(&format!(
                        "Vous avez quitté la partie ! Score : {}",
                        score
                    ));
                    in_configuration = true;
                }

                update_model_vaisseau(&mut vaisseau);
                update_asteroids(&mut liste_asteroid);
                update_missiles(&mut missiles);

                // Tirer un missile si nécessaire
                if let Some(missile) = vaisseau.fire_missile(get_time()) {
                    play_game_sound(&sounds.shoot, false, 0.1);
                    missiles.push(missile);
                }

                check_vaisseau_asteroids(
                    &mut vaisseau,
                    &mut liste_asteroid,
                    hauteur_vaisseau,
                    level_size,
                    gravite_dist,
                    Some(&sounds.ship_hit),
                );
                check_missiles_asteroids(
                    &mut missiles,
                    &mut liste_asteroid,
                    rayon_missile,
                    level_size,
                    Some(&sounds.asteroid_hit),
                    &mut score,
                );

                // Si la taille de l'écran a changé, ajuster l'échelle des objets
                if last_screen_size != (screen_width(), screen_height()) {
                    update_scale(
                        &mut hauteur_vaisseau,
                        &mut rayon_missile,
                        &mut level_size,
                        &mut last_screen_size,
                        &mut gravite_dist,
                        false,
                    );
                }

                // Affichage des objets et du score
                draw_background(&textures.sprite_background);
                draw_vaisseau(&vaisseau, &textures.sprite_vaisseau, hauteur_vaisseau);
                draw_missiles(&missiles, rayon_missile, &textures.sprite_meteor);
                draw_asteroids_batched(
                    &mut liste_asteroid,
                    level_size,
                    [
                        &textures.sprite_asteroid_1,
                        &textures.sprite_asteroid_2,
                        &textures.sprite_asteroid_3,
                    ],
                );

                let shield_text = format!("Bouclier: {:.0}", vaisseau.get_shield());
                let score_text = format!("Score: {}", score);

                draw_text(&shield_text, 10.0, 30.0, 30.0, WHITE);
                draw_text(&score_text, 10.0, 70.0, 30.0, WHITE);
            }

            next_frame().await
        }

        // Gestion du temps d'exécution de chaque frame
        let elapsed_frame_time = (get_time() - start_time) as f32;
        if elapsed_frame_time < frame_duration {
            let sleep_time = frame_duration - elapsed_frame_time;
            std::thread::sleep(std::time::Duration::from_secs_f32(sleep_time));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Teste le comportement de le vaisseau avec plusieurs niveaux d'astéroïdes.
    ///
    /// Ce test simule différentes positions d'astéroïdes à différents niveaux,
    /// et vérifie si le vaisseau réagit correctement en termes de bouclier et de collisions.
    /// Il vérifie également si la vitesse de le vaisseau est modifiée par la gravité des astéroïdes.
    #[test]
    fn test_vaisseau_asteroids_multiple_levels() {
        let level_size = (40., 20., 10.);

        // Scénarios à tester
        let scenarios = vec![
            (
                vec![
                    Vec2::new(10., 10.),
                    Vec2::new(70., 100.),
                    Vec2::new(500., 250.),
                ],
                4.,
                "Le bouclier de le vaisseau n'est pas égal à 4 avec cette position",
            ),
            (
                vec![
                    Vec2::new(40., 200.),
                    Vec2::new(10., 10.),
                    Vec2::new(60., 60.),
                ],
                3.,
                "Le bouclier de le vaisseau n'est pas égal à 3 avec cette position",
            ),
            (
                vec![
                    Vec2::new(100., 50.),
                    Vec2::new(30., 120.),
                    Vec2::new(10., 10.),
                ],
                2.,
                "Le bouclier de le vaisseau n'est pas égal à 2 avec cette position",
            ),
        ];
        for (iteration, (positions, expected_shield, message)) in scenarios.into_iter().enumerate()
        {
            // Préparation de le vaisseau et des astéroïdes
            let mut vaisseau = Vaisseau::new(Some(Vec2::new(10., 10.)), Some(0.));
            let initiale_speed = vaisseau.get_speed();
            let asteroid1 = Asteroid::new(1, Vec2::ZERO, level_size, Some(positions[0]));
            let asteroid2 = Asteroid::new(2, Vec2::ZERO, level_size, Some(positions[1]));
            let asteroid3 = Asteroid::new(3, Vec2::ZERO, level_size, Some(positions[2]));

            let mut asteroids = vec![asteroid1, asteroid2, asteroid3];

            // Appel de la fonction
            check_vaisseau_asteroids(
                &mut vaisseau,
                &mut asteroids[..],
                30.,
                level_size,
                30.,
                None,
            );
            // Déterminer les indices des astéroïdes à vérifier
            let check_indices = match iteration {
                0 => (1, 2),         // Vérifier les astéroïdes 2 et 3
                1 => (0, 2),         // Vérifier les astéroïdes 1 et 3
                2 => (0, 1),         // Vérifier les astéroïdes 1 et 2
                _ => unreachable!(), // Le test ne devrait jamais dépasser 3 itérations
            };
            // Assertions
            assert!(vaisseau.get_shield() == expected_shield, "{}", message);
            assert!(
                !asteroids[check_indices.0].get_collided(),
                "L'astéroïde {} ne doit pas être en collision",
                check_indices.0
            );
            assert!(
                !asteroids[check_indices.1].get_collided(),
                "L'astéroïde {} doit être en collision",
                check_indices.1
            );
            assert!(
                asteroids[iteration].get_collided(),
                "L'astéroïde {} doit être en collision",
                iteration
            );
            assert!(
                (vaisseau.get_speed() == initiale_speed),
                "le vaisseau doit c'est fait pris la gravité de l'asteroid"
            )
        }
    }

    /// Teste la collision entre un missile et un astéroïde.
    ///
    /// Ce test vérifie que les missiles interagissent correctement avec les astéroïdes,
    /// en s'assurant qu'après avoir touché un astéroïde, celui-ci est détruit et que le score
    /// est mis à jour correctement.
    #[test]
    fn test_collision_missile_asteroid() {
        let level_size = (40., 20., 10.);

        let asteroid = Asteroid::new(3, Vec2::ZERO, level_size, Some(Vec2::new(0., 0.)));

        let rayon_missile = 7.0;

        let mut score = 0;

        let mut asteroids = vec![asteroid];

        let mut missiles = Vec::new();
        for i in 0..3 {
            let position = Vec2::new(i as f32 * 100., i as f32 * 100.);
            missiles.push(Missile::new(position, 0.));
        }

        for i in 0..3 {
            check_missiles_asteroids(
                &mut missiles,
                &mut asteroids,
                rayon_missile,
                level_size,
                None,
                &mut score,
            );
            if let Some(last_asteroid) = asteroids.last_mut() {
                let k = (i + 1) as f32;
                last_asteroid.set_position(Vec2::new(k * 100., k * 100.));
            }
        }
        for (i, ast) in asteroids.iter_mut().enumerate() {
            assert!(
                ast.get_level() == (2 - i as u8),
                "Le niveau de l'asteroid devrait être {}",
                i
            )
        }
        assert!(
            missiles.is_empty(),
            "Il es sensé n'y avoir aucun missile restant"
        );
        assert!(
            score == 60,
            "Le score devrait être de 60 (level 3 : 30 + level 2 : 20 + level 1 : 10)"
        )
    }
    /// Teste la réinitialisation du jeu.
    ///
    /// Ce test simule la réinitialisation de l'état du jeu, en vérifiant que :
    /// 1. Le score est réinitialisé à zéro.
    /// 2. La position de le vaisseau est réinitialisée à (0, 0).
    /// 3. Le nombre d'astéroïdes est réinitialisé à 5.
    #[test]
    fn test_reset_game() {
        let position = Some(Vec2::new(0., 0.));
        let mut vaisseau = Vaisseau::new(position, Some(0.));
        let mut liste_asteroid = vec![Asteroid::new(3, Vec2::ZERO, (40.0, 20.0, 10.0), position)];
        let mut missiles = Vec::new();
        let mut score = 100; // Un score initial non nul

        reset_game(
            &mut liste_asteroid,
            &mut vaisseau,
            &mut missiles,
            (40.0, 20.0, 10.0),
            5,
            1.0,
            &mut score,
            true,
        );

        // Vérifiez si le score a été réinitialisé
        assert!(score == 0, "Initalement le score doit être a 0");

        // Vérifiez si le vaisseau est réinitialisé (par exemple, sa position)
        assert!(
            vaisseau.get_position() == Vec2::ZERO,
            "On simule que le vaisseau spawn en 0 0 donc il doit y être"
        );

        // Vérifiez si les astéroïdes ont été réinitialisés
        assert!(
            liste_asteroid.len() == 5,
            "Le nombre d'asteroid doit être 5"
        ); // Nombre d'astéroïdes après réinitialisation
        assert!(missiles.len() == 0, "Il doit y avoir 0 missile"); // Les missiles doivent être vides
    }

    /// Teste la trajectoire du missile.
    ///
    /// Ce test simule la trajectoire d'un missile en vérifiant que :
    /// 1. Le missile suit correctement la trajectoire calculée sur 1 et 2 secondes.
    /// 2. Le missile reste à l'écran ou sort correctement de l'écran après un certain temps.
    #[test]
    fn test_missile_trajectory() {
        let vaisseau_initial_position = Vec2::new(100.0, 100.0);
        let vaisseau_rotation = 135.0;

        let mut missile = Missile::new(vaisseau_initial_position, vaisseau_rotation);

        let missile_velocity = missile.get_speed();
        let expected_position_after_1s = vaisseau_initial_position + missile_velocity;

        let screen_width = 101.;
        let screen_height = 102.;

        missile.update_position();

        assert!(
            missile.get_position() == expected_position_after_1s,
            "Il ne suit plus la trajectoire après 1s"
        );

        assert!(
            !missile.is_off_screen(screen_width, screen_height),
            "Le missile est hors de l'écran après 1 seconde"
        );

        let expected_position_after_2s = expected_position_after_1s + missile_velocity;
        missile.update_position();

        assert!(
            missile.get_position() == expected_position_after_2s,
            "Il ne suit plus la trajectoire après 2s"
        );

        assert!(
            missile.is_off_screen(screen_width, screen_height),
            "Le missile devrait être hors de l'écran après plusieurs déplacements"
        );
    }
    /// Teste l'application du facteur d'échelle pour ajuster la taille des éléments.
    ///
    /// Ce test vérifie que les différentes tailles, telles que la hauteur de le vaisseau,
    /// le rayon du missile et la taille du niveau, sont mises à l'échelle correctement
    /// en fonction du nouveau facteur d'échelle de l'écran.
    #[test]
    fn test_scale() {
        let mut hauteur_vaisseau: f32 = 30.;
        let mut rayon_missile: f32 = 7.;
        let mut level_size: (f32, f32, f32) = (40., 20., 10.);
        let mut gravite_dist: f32 = 30.;
        let mut last_screen_size: (f32, f32) = (400., 300.);

        let width_scale = 500. / last_screen_size.0;
        let height_scale = 400. / last_screen_size.1;

        let scale_factor = (width_scale + height_scale) / 2.;

        update_scale(
            &mut hauteur_vaisseau,
            &mut rayon_missile,
            &mut level_size,
            &mut last_screen_size,
            &mut gravite_dist,
            true,
        );

        assert_eq!(hauteur_vaisseau, (30. * scale_factor));
        assert_eq!(rayon_missile, 7. * scale_factor);
        assert_eq!(level_size.0, 40. * scale_factor);
        assert_eq!(level_size.1, 20. * scale_factor);
        assert_eq!(level_size.2, 10. * scale_factor);
        assert_eq!(gravite_dist, 30. * scale_factor);
        assert_eq!(last_screen_size, (500., 400.))
    }
}
