Este proyecto es una simulación variante del sistema solar, en la que los planetas giran alrededor del sol, rotan sobre su eje, y se puede explorar la escena en 3D. La simulación incluye un TIE Fighter como nave espacial y un skybox de estrellas que crea un fondo espacial dinámico.

Requisitos
	•	Rust: La simulación está escrita en Rust, por lo que necesitarás tener instalado Rust￼ para compilar y ejecutar el proyecto.

Características
	•	Planetas en órbita: El sol y los planetas se renderizan y se mueven a lo largo de sus órbitas alrededor del sol.
	•	Rotación de los planetas: Cada planeta rota sobre su eje, y el patrón de rotación depende del tipo de planeta (rocoso o gaseoso).
	•	Cámara 3D: Puedes mover la cámara a través del plano eclíptico (WASD), hacer zoom (Q/E), y teletransportarte instantáneamente entre planetas con la tecla T.
	•	Skybox de estrellas: El fondo está compuesto por un skybox que representa un campo estrellado dinámico.
	•	Nave TIE Fighter: Se incluye una nave espacial tipo TIE Fighter que sigue a la cámara.

Controles
	•	W: Mover la cámara hacia adelante.
	•	S: Mover la cámara hacia atrás.
	•	A: Mover la cámara hacia la izquierda.
	•	D: Mover la cámara hacia la derecha.
	•	Q: Acercar la cámara (zoom).
	•	E: Alejar la cámara (zoom).
	•	Esc: Salir del programa.

Instalación
	1.	Clona el repositorio:

git clone https://github.com/tu_usuario/sistema_solar.git


	2.	Navega a la carpeta del proyecto:

cd sistema_solar


	3.	Compila el proyecto:

cargo build --release


	4.	Ejecuta la simulación:

cargo run --release



Mejoras Futuras
	•	Interacción con la nave: Permitir el control de la nave TIE Fighter por el usuario.
	•	Instant Warping: Implementar un sistema más avanzado de teletransporte entre los planetas.
	•	Optimización: Mejorar la fluidez y la eficiencia del renderizado para soportar más planetas y mejorar el rendimiento.

