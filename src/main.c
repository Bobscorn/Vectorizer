
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <png.h>
#include <nanosvg.h>
#include <jpeglib.h>
#include "entrypoint.h"

#include "vectorize.h"
#include "converter.h"

int main(int argc, char* argv[]) {	
	//entrypoint(argc, argv);
	printf("Hello there\n");
	// TEMPORARY
	if (argc < 2)
	{
		printf("Please supply a file path\n");
		return 0;
	}

	printf("Converting image...\n");
	image my_image = convert_png_to_image(argv[1]);

	printf("Generating Node Map\n");
	node_map map = generate_node_map(my_image, (node_map_options){ 4 });

	printf("Writing image back to disk\n");
	write_image_to_file(my_image, "bob.ppm");
	printf("Writing node map to disk\n");
	write_node_map_to_file(map, "bob_map.bmp");


	return 0;
}