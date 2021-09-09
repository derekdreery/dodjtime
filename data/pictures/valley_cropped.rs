// Image size 83x39
const VALLEY_IMG: [u8; 6474] = [220, 194, 220, 226, 220, 226, 220, 226, 220, 194, 220, 97, 228, 196, 124, 32, 180, 34, 212, 35, 220, 98, 132, 0, 213, 133, 229, 1, 220, 225, 228, 163, 220, 162, 212, 193, 195, 161, 171, 99, 220, 163, 220, 65, 179, 98, 122, 131, 131, 37, 204, 68, 212, 130, 187, 194, 179, 98, 114, 131, 172, 8, 188, 234, 212, 196, 220, 193, 220, 226, 220, 226, 220, 194, 220, 193, 220, 194, 220, 225, 221, 0, 220, 226, 220, 194, 220, 194, 220, 194, 220, 225, 220, 225, 220, 193, 220, 194, 220, 226, 220, 226, 220, 226, 220, 226, 220, 129, 147, 0, 146, 226, 32, 128, 66, 105, 107, 76, 16, 162, 130, 197, 155, 5, 180, 40, 221, 236, 229, 238, 237, 68, 163, 161, 81, 0, 130, 101, 81, 1, 195, 167, 203, 197, 130, 98, 195, 197, 245, 71, 138, 98, 195, 166, 245, 102, 138, 98, 195, 166, 89, 0, 165, 228, 105, 224, 229, 2, 228, 225, 228, 225, 228, 225, 228, 226, 195, 226, 179, 130, 90, 192, 196, 132, 212, 130, 140, 64, 213, 101, 220, 194, 229, 2, 89, 96, 73, 128, 65, 160, 65, 160, 228, 230, 221, 2, 229, 1, 220, 97, 203, 225, 187, 129, 195, 226, 220, 163, 228, 226, 228, 225, 228, 225, 196, 2, 155, 36, 114, 163, 212, 229, 220, 161, 228, 226, 228, 226, 228, 226, 229, 3, 229, 33, 229, 33, 229, 2, 228, 226, 228, 194, 228, 225, 228, 226, 228, 225, 228, 225, 228, 225, 229, 2, 228, 226, 220, 225, 228, 225, 228, 226, 221, 2, 229, 2, 32, 160, 8, 195, 74, 73, 107, 140, 16, 163, 16, 227, 187, 229, 171, 4, 155, 37, 188, 200, 229, 36, 155, 130, 139, 99, 81, 64, 81, 33, 130, 101, 187, 198, 138, 131, 187, 198, 195, 164, 130, 67, 195, 198, 203, 197, 130, 98, 88, 224, 165, 196, 109, 0, 97, 224, 229, 1, 229, 2, 229, 1, 229, 2, 228, 226, 228, 98, 228, 225, 221, 1, 204, 0, 179, 130, 83, 0, 221, 163, 229, 96, 81, 160, 91, 164, 117, 69, 117, 132, 91, 225, 57, 192, 212, 229, 220, 194, 220, 130, 220, 97, 229, 3, 229, 2, 229, 2, 237, 66, 237, 65, 237, 64, 204, 1, 195, 225, 221, 34, 237, 2, 228, 225, 237, 34, 220, 129, 228, 194, 228, 226, 229, 2, 229, 1, 229, 1, 229, 1, 237, 33, 237, 65, 237, 65, 237, 65, 229, 2, 229, 2, 229, 1, 229, 2, 229, 1, 229, 2, 228, 225, 228, 130, 32, 128, 74, 73, 24, 163, 74, 73, 107, 108, 16, 195, 66, 72, 16, 195, 188, 102, 228, 98, 253, 66, 253, 97, 147, 129, 139, 131, 139, 35, 81, 0, 80, 224, 130, 70, 138, 68, 130, 100, 130, 69, 130, 100, 130, 99, 73, 0, 81, 1, 165, 195, 116, 224, 158, 64, 97, 224, 229, 1, 228, 193, 237, 34, 229, 1, 228, 193, 228, 194, 237, 2, 228, 225, 237, 2, 245, 66, 245, 96, 237, 128, 97, 224, 66, 34, 109, 164, 118, 2, 117, 194, 109, 164, 109, 101, 73, 192, 212, 228, 228, 225, 237, 34, 237, 34, 237, 34, 245, 67, 245, 97, 237, 96, 245, 66, 237, 34, 237, 33, 245, 34, 237, 1, 228, 192, 236, 225, 228, 193, 237, 2, 237, 34, 237, 34, 237, 1, 237, 2, 245, 66, 245, 98, 237, 97, 245, 97, 237, 65, 237, 33, 237, 33, 229, 1, 228, 193, 229, 2, 237, 2, 228, 193, 237, 2, 32, 160, 115, 77, 41, 133, 66, 73, 66, 72, 41, 102, 107, 140, 16, 162, 229, 135, 245, 129, 237, 33, 236, 193, 237, 33, 155, 0, 131, 68, 139, 67, 139, 100, 80, 224, 64, 224, 81, 0, 73, 0, 81, 0, 72, 225, 83, 4, 158, 3, 158, 96, 117, 0, 199, 131, 97, 224, 245, 98, 253, 130, 237, 65, 245, 65, 253, 130, 253, 130, 245, 131, 245, 33, 236, 225, 237, 34, 253, 66, 245, 98, 65, 160, 83, 163, 125, 196, 109, 162, 101, 226, 76, 65, 50, 224, 33, 192, 237, 167, 245, 131, 245, 98, 245, 33, 236, 225, 245, 33, 245, 98, 245, 66, 245, 97, 245, 65, 245, 161, 253, 161, 245, 98, 253, 98, 237, 65, 237, 65, 245, 162, 245, 163, 245, 130, 245, 33, 236, 226, 245, 2, 245, 98, 245, 97, 245, 98, 245, 98, 245, 129, 253, 161, 245, 98, 245, 65, 245, 98, 245, 98, 253, 98, 253, 131, 24, 192, 107, 108, 74, 104, 49, 134, 41, 101, 74, 106, 107, 108, 16, 194, 229, 198, 253, 131, 245, 65, 245, 66, 245, 98, 245, 97, 253, 162, 245, 162, 245, 98, 236, 192, 223, 34, 158, 32, 116, 224, 117, 33, 158, 33, 158, 32, 199, 97, 199, 129, 117, 32, 158, 33, 98, 0, 245, 67, 245, 98, 237, 33, 237, 65, 245, 66, 253, 98, 245, 65, 236, 224, 245, 97, 245, 98, 237, 97, 253, 131, 65, 160, 91, 196, 117, 228, 109, 226, 109, 194, 76, 32, 109, 100, 50, 0, 73, 96, 237, 133, 245, 98, 236, 225, 245, 66, 237, 65, 245, 65, 253, 162, 245, 97, 245, 131, 253, 161, 245, 97, 245, 65, 245, 98, 237, 33, 237, 65, 245, 66, 245, 98, 245, 65, 228, 224, 245, 66, 245, 98, 245, 97, 253, 161, 245, 98, 245, 130, 253, 161, 245, 129, 245, 98, 245, 98, 245, 98, 245, 98, 245, 98, 245, 98, 24, 160, 66, 73, 115, 76, 107, 108, 107, 139, 107, 108, 74, 104, 16, 164, 229, 230, 245, 129, 245, 98, 245, 98, 245, 98, 245, 97, 245, 98, 245, 130, 245, 160, 228, 194, 223, 34, 117, 0, 180, 35, 108, 6, 100, 70, 150, 67, 158, 32, 158, 64, 158, 65, 166, 64, 97, 224, 245, 97, 253, 162, 245, 97, 237, 33, 237, 1, 237, 33, 245, 67, 245, 97, 253, 65, 245, 162, 253, 129, 73, 128, 65, 224, 66, 163, 76, 34, 117, 163, 109, 226, 109, 132, 117, 226, 58, 161, 238, 203, 73, 32, 229, 101, 237, 66, 245, 98, 253, 130, 253, 161, 253, 161, 245, 97, 245, 98, 245, 97, 245, 97, 245, 97, 253, 162, 245, 97, 237, 33, 237, 1, 245, 2, 245, 66, 245, 98, 245, 97, 253, 130, 253, 130, 253, 193, 245, 96, 245, 98, 245, 98, 245, 130, 245, 129, 245, 162, 245, 97, 245, 65, 237, 1, 245, 98, 245, 0, 40, 224, 41, 167, 41, 134, 49, 134, 41, 134, 24, 133, 229, 72, 245, 97, 245, 97, 229, 129, 245, 130, 245, 98, 245, 65, 237, 1, 245, 2, 237, 98, 228, 193, 236, 224, 212, 98, 195, 194, 146, 162, 116, 6, 116, 194, 158, 65, 158, 96, 199, 129, 117, 0, 97, 224, 245, 160, 253, 160, 245, 129, 245, 65, 237, 34, 236, 226, 236, 193, 89, 0, 73, 96, 237, 165, 97, 128, 65, 97, 65, 98, 57, 192, 66, 162, 83, 227, 117, 163, 110, 1, 68, 97, 66, 194, 65, 192, 57, 160, 228, 229, 245, 194, 253, 160, 253, 161, 253, 161, 245, 97, 245, 98, 237, 65, 245, 66, 245, 97, 245, 129, 253, 160, 245, 129, 245, 65, 237, 34, 236, 225, 237, 33, 245, 130, 253, 162, 253, 162, 245, 98, 245, 97, 245, 66, 237, 33, 245, 66, 245, 98, 253, 162, 253, 161, 245, 97, 245, 65, 237, 33, 244, 161, 229, 66, 253, 131, 32, 160, 41, 165, 33, 166, 16, 163, 221, 134, 245, 32, 253, 66, 253, 98, 253, 130, 253, 161, 245, 129, 245, 98, 245, 65, 237, 1, 245, 1, 236, 225, 228, 225, 212, 66, 173, 192, 116, 192, 199, 162, 199, 96, 116, 192, 158, 98, 117, 0, 158, 33, 97, 224, 253, 130, 245, 98, 245, 98, 245, 97, 245, 98, 237, 98, 105, 192, 73, 129, 147, 200, 97, 128, 106, 166, 155, 232, 114, 196, 73, 162, 57, 193, 66, 162, 50, 193, 50, 161, 50, 130, 49, 160, 57, 194, 213, 39, 229, 98, 245, 162, 253, 162, 245, 130, 245, 98, 245, 97, 245, 97, 245, 98, 245, 98, 245, 161, 245, 162, 245, 98, 245, 98, 245, 97, 245, 98, 245, 98, 245, 98, 245, 98, 245, 98, 245, 130, 245, 98, 245, 97, 245, 97, 245, 65, 245, 97, 245, 130, 253, 162, 245, 98, 245, 97, 245, 98, 245, 66, 237, 33, 237, 64, 245, 163, 65, 98, 33, 133, 41, 134, 49, 134, 221, 103, 174, 0, 158, 96, 229, 224, 245, 130, 245, 98, 245, 98, 245, 98, 245, 98, 245, 129, 253, 66, 73, 96, 81, 65, 73, 65, 73, 96, 81, 97, 173, 193, 150, 64, 207, 131, 158, 32, 117, 0, 199, 130, 97, 224, 245, 98, 245, 98, 245, 130, 253, 162, 245, 130, 245, 97, 81, 96, 114, 167, 205, 172, 156, 8, 156, 39, 114, 196, 122, 166, 114, 133, 114, 165, 114, 230, 65, 161, 65, 130, 106, 198, 114, 165, 114, 165, 81, 96, 237, 68, 237, 33, 245, 66, 245, 98, 237, 33, 236, 225, 237, 1, 245, 98, 245, 65, 245, 98, 245, 98, 245, 98, 245, 130, 253, 162, 245, 130, 245, 98, 237, 65, 245, 66, 245, 65, 245, 66, 245, 33, 245, 33, 236, 226, 245, 2, 245, 66, 237, 33, 245, 66, 245, 97, 253, 130, 253, 162, 253, 161, 244, 225, 204, 167, 49, 131, 74, 72, 140, 110, 140, 77, 107, 108, 41, 132, 116, 196, 199, 131, 158, 32, 229, 194, 245, 98, 245, 161, 245, 161, 245, 161, 245, 100, 73, 33, 195, 230, 253, 135, 237, 170, 196, 6, 196, 6, 73, 65, 158, 2, 199, 97, 150, 32, 150, 64, 199, 98, 98, 0, 245, 98, 245, 97, 245, 97, 245, 97, 245, 98, 245, 129, 89, 64, 147, 200, 164, 104, 114, 164, 122, 196, 147, 198, 205, 140, 205, 237, 156, 71, 122, 229, 114, 102, 114, 134, 81, 129, 114, 134, 155, 232, 73, 98, 237, 69, 237, 34, 237, 1, 245, 66, 237, 33, 237, 65, 237, 33, 236, 224, 245, 66, 245, 98, 245, 97, 245, 97, 245, 97, 245, 97, 245, 98, 245, 98, 245, 98, 237, 33, 237, 1, 236, 193, 245, 2, 236, 226, 236, 226, 236, 225, 245, 34, 245, 66, 237, 33, 245, 65, 245, 65, 245, 130, 245, 162, 228, 64, 171, 100, 57, 99, 107, 109, 140, 78, 181, 209, 140, 46, 41, 165, 172, 6, 195, 131, 229, 99, 245, 65, 245, 97, 245, 66, 253, 130, 253, 161, 73, 97, 170, 194, 253, 136, 237, 168, 254, 237, 253, 136, 203, 231, 138, 68, 89, 65, 158, 96, 158, 32, 207, 130, 125, 0, 97, 193, 253, 130, 253, 160, 245, 129, 245, 65, 245, 66, 236, 226, 81, 96, 106, 196, 122, 197, 114, 133, 81, 98, 114, 198, 156, 6, 205, 139, 205, 171, 205, 172, 164, 40, 114, 196, 73, 98, 147, 234, 156, 38, 73, 161, 221, 68, 245, 65, 245, 97, 237, 65, 237, 64, 237, 33, 237, 33, 245, 66, 245, 97, 245, 129, 253, 161, 253, 160, 245, 129, 245, 65, 245, 66, 237, 33, 245, 65, 245, 66, 245, 34, 236, 226, 236, 225, 236, 225, 236, 225, 245, 2, 236, 226, 245, 33, 245, 98, 245, 98, 245, 65, 237, 33, 245, 66, 236, 225, 178, 192, 65, 99, 66, 41, 107, 109, 140, 78, 132, 75, 57, 101, 221, 71, 253, 66, 253, 98, 245, 193, 253, 161, 245, 64, 237, 97, 245, 98, 81, 65, 170, 162, 195, 229, 253, 168, 196, 5, 253, 169, 204, 37, 162, 195, 73, 33, 1, 68, 158, 65, 158, 32, 199, 161, 105, 224, 245, 193, 245, 98, 245, 98, 245, 33, 237, 1, 245, 130, 220, 226, 81, 96, 131, 7, 122, 166, 73, 98, 73, 97, 122, 198, 114, 164, 156, 7, 156, 7, 114, 195, 73, 128, 114, 135, 147, 231, 81, 192, 221, 70, 237, 66, 245, 131, 253, 161, 245, 192, 245, 162, 237, 65, 245, 65, 245, 130, 245, 161, 253, 160, 253, 161, 245, 98, 245, 98, 245, 33, 237, 1, 245, 66, 237, 33, 237, 33, 236, 226, 236, 225, 236, 225, 237, 2, 236, 225, 236, 226, 236, 226, 245, 34, 245, 98, 245, 97, 245, 65, 245, 33, 236, 226, 173, 224, 133, 64, 98, 128, 49, 132, 66, 72, 74, 72, 41, 199, 147, 196, 171, 225, 245, 194, 253, 161, 253, 161, 245, 98, 229, 96, 81, 65, 73, 97, 138, 36, 170, 163, 162, 227, 162, 194, 162, 226, 162, 195, 170, 194, 130, 68, 130, 68, 73, 98, 117, 0, 158, 66, 199, 129, 105, 192, 245, 66, 237, 33, 245, 66, 236, 225, 237, 34, 245, 98, 245, 33, 131, 1, 73, 97, 73, 129, 238, 172, 81, 128, 81, 130, 73, 98, 73, 130, 73, 128, 65, 32, 114, 166, 122, 165, 122, 198, 188, 168, 89, 64, 237, 228, 253, 161, 245, 162, 237, 65, 245, 98, 245, 98, 237, 65, 237, 65, 245, 66, 245, 97, 245, 66, 237, 33, 245, 66, 236, 225, 237, 34, 245, 98, 237, 33, 236, 225, 236, 226, 236, 226, 237, 2, 236, 225, 236, 225, 237, 1, 245, 66, 245, 97, 237, 65, 237, 33, 245, 66, 237, 1, 237, 2, 245, 66, 91, 64, 66, 160, 16, 193, 41, 134, 41, 103, 16, 163, 90, 35, 155, 162, 237, 67, 245, 97, 245, 66, 237, 65, 81, 34, 204, 70, 245, 137, 254, 236, 254, 205, 246, 236, 246, 237, 246, 235, 246, 236, 253, 136, 245, 167, 253, 168, 196, 6, 73, 64, 150, 32, 158, 64, 97, 224, 253, 130, 245, 97, 245, 65, 237, 65, 245, 98, 245, 99, 237, 64, 138, 160, 122, 228, 57, 1, 230, 103, 204, 166, 81, 129, 106, 134, 114, 198, 122, 198, 122, 166, 114, 166, 73, 97, 65, 96, 246, 170, 73, 160, 229, 231, 253, 67, 237, 65, 237, 65, 236, 225, 237, 33, 237, 65, 237, 65, 245, 65, 245, 129, 253, 130, 245, 98, 245, 65, 237, 65, 245, 98, 245, 98, 237, 33, 237, 33, 236, 225, 237, 1, 237, 33, 237, 33, 245, 34, 245, 97, 245, 130, 245, 161, 253, 161, 237, 65, 245, 98, 245, 98, 195, 192, 187, 164, 187, 194, 57, 224, 16, 193, 41, 134, 66, 105, 8, 131, 90, 35, 155, 130, 237, 68, 253, 128, 253, 129, 81, 65, 204, 5, 245, 169, 204, 5, 196, 39, 81, 64, 73, 66, 73, 65, 81, 33, 81, 32, 73, 96, 130, 101, 245, 136, 253, 105, 196, 7, 81, 97, 158, 64, 97, 192, 253, 65, 245, 97, 237, 33, 253, 162, 253, 162, 245, 33, 245, 34, 139, 0, 130, 226, 131, 4, 139, 97, 146, 225, 130, 196, 139, 101, 139, 36, 139, 36, 131, 3, 131, 3, 130, 228, 229, 168, 237, 163, 221, 99, 237, 34, 237, 65, 245, 2, 245, 33, 245, 33, 245, 65, 237, 65, 245, 130, 253, 161, 253, 162, 245, 97, 245, 98, 237, 65, 253, 162, 253, 162, 237, 33, 245, 66, 245, 34, 245, 34, 245, 66, 245, 66, 245, 65, 245, 65, 245, 130, 245, 130, 253, 162, 245, 97, 237, 33, 245, 97, 245, 161, 245, 131, 187, 192, 138, 129, 33, 64, 16, 195, 66, 73, 66, 71, 24, 196, 90, 33, 171, 162, 245, 162, 245, 193, 245, 98, 65, 128, 138, 37, 204, 6, 130, 67, 73, 65, 146, 6, 24, 32, 32, 96, 105, 101, 24, 195, 24, 193, 73, 64, 81, 97, 204, 38, 196, 39, 65, 96, 158, 33, 97, 224, 237, 195, 245, 65, 253, 131, 245, 131, 237, 65, 245, 34, 228, 194, 220, 195, 146, 193, 138, 162, 123, 2, 139, 4, 131, 34, 131, 2, 131, 33, 130, 226, 131, 2, 130, 226, 221, 103, 245, 131, 253, 130, 245, 33, 236, 225, 236, 194, 244, 192, 236, 226, 237, 2, 245, 66, 245, 130, 253, 161, 245, 96, 245, 97, 245, 98, 245, 65, 253, 130, 245, 130, 237, 33, 245, 34, 236, 193, 236, 225, 236, 193, 236, 226, 237, 2, 245, 66, 245, 130, 253, 161, 245, 65, 245, 65, 237, 65, 237, 65, 253, 130, 253, 131, 245, 98, 237, 33, 138, 128, 130, 129, 24, 160, 66, 105, 107, 108, 24, 131, 155, 229, 245, 194, 245, 65, 245, 97, 245, 97, 81, 34, 65, 97, 73, 129, 81, 65, 88, 36, 246, 115, 114, 37, 189, 53, 16, 96, 1, 69, 254, 115, 24, 194, 73, 34, 130, 100, 130, 100, 81, 65, 158, 64, 97, 224, 230, 77, 229, 36, 245, 98, 245, 33, 237, 65, 245, 66, 245, 98, 237, 32, 220, 163, 229, 35, 147, 1, 131, 3, 131, 3, 139, 2, 130, 227, 138, 195, 221, 6, 229, 36, 245, 131, 245, 65, 245, 67, 245, 98, 245, 98, 245, 65, 236, 225, 245, 2, 245, 66, 245, 65, 237, 65, 237, 65, 245, 98, 228, 224, 245, 66, 237, 33, 237, 33, 237, 65, 237, 33, 245, 66, 245, 98, 237, 33, 236, 225, 237, 1, 245, 65, 245, 97, 245, 97, 245, 97, 237, 33, 236, 225, 245, 33, 237, 1, 245, 66, 245, 97, 245, 66, 237, 98, 245, 129, 40, 160, 8, 194, 74, 73, 107, 140, 16, 163, 16, 227, 229, 134, 237, 96, 236, 194, 245, 97, 237, 33, 245, 97, 245, 66, 112, 39, 254, 52, 254, 50, 106, 6, 247, 223, 254, 83, 254, 82, 246, 115, 1, 69, 10, 169, 73, 98, 81, 99, 166, 0, 109, 0, 97, 192, 196, 201, 237, 100, 245, 98, 245, 65, 237, 34, 236, 193, 245, 66, 245, 97, 245, 97, 245, 97, 237, 66, 245, 133, 245, 131, 245, 133, 237, 101, 237, 133, 253, 195, 245, 97, 245, 99, 237, 97, 237, 65, 236, 225, 237, 33, 237, 65, 237, 65, 245, 98, 253, 161, 253, 161, 253, 161, 253, 162, 245, 98, 245, 98, 245, 65, 245, 66, 245, 97, 245, 66, 237, 33, 228, 225, 245, 98, 245, 97, 245, 98, 245, 65, 245, 98, 245, 130, 245, 161, 253, 162, 245, 97, 253, 163, 245, 130, 245, 98, 245, 66, 245, 65, 245, 33, 236, 225, 32, 160, 74, 73, 24, 196, 74, 73, 107, 108, 16, 195, 66, 72, 16, 195, 221, 104, 245, 98, 245, 65, 245, 65, 245, 98, 237, 66, 245, 33, 104, 135, 246, 116, 246, 82, 254, 50, 254, 51, 246, 115, 243, 238, 2, 169, 1, 166, 1, 68, 166, 97, 117, 64, 150, 33, 97, 192, 236, 194, 228, 193, 245, 65, 245, 34, 236, 224, 237, 33, 245, 66, 245, 98, 253, 98, 253, 130, 253, 162, 245, 192, 245, 162, 245, 98, 245, 97, 245, 66, 245, 33, 236, 162, 245, 66, 245, 34, 236, 225, 237, 1, 245, 66, 237, 65, 245, 66, 245, 130, 253, 161, 253, 161, 253, 161, 245, 98, 245, 97, 245, 97, 237, 65, 236, 224, 245, 66, 245, 34, 236, 225, 237, 1, 245, 98, 245, 98, 245, 97, 253, 162, 245, 160, 253, 161, 253, 162, 245, 98, 245, 98, 245, 66, 237, 65, 236, 225, 245, 66, 237, 1, 236, 193, 237, 34, 32, 192, 107, 108, 41, 133, 66, 73, 66, 72, 41, 102, 107, 140, 8, 225, 221, 134, 245, 65, 237, 33, 236, 224, 245, 34, 245, 33, 236, 225, 237, 1, 112, 72, 254, 51, 246, 115, 254, 83, 243, 237, 1, 69, 9, 37, 3, 174, 1, 4, 158, 64, 117, 0, 191, 131, 97, 224, 237, 66, 245, 65, 237, 98, 245, 97, 253, 99, 253, 131, 245, 129, 245, 34, 236, 225, 237, 34, 245, 97, 245, 98, 245, 97, 245, 66, 253, 160, 245, 161, 245, 98, 245, 97, 245, 66, 245, 97, 245, 130, 253, 130, 245, 130, 245, 33, 237, 1, 237, 1, 245, 97, 245, 129, 245, 130, 245, 130, 245, 160, 253, 161, 245, 130, 245, 130, 245, 97, 245, 97, 245, 130, 253, 162, 245, 130, 245, 98, 245, 98, 253, 130, 245, 66, 245, 98, 245, 97, 245, 98, 245, 97, 245, 97, 237, 65, 245, 98, 237, 33, 245, 97, 253, 162, 253, 131, 32, 224, 107, 108, 74, 105, 49, 134, 41, 101, 74, 106, 107, 108, 16, 194, 229, 197, 245, 161, 253, 66, 237, 65, 245, 97, 245, 97, 245, 98, 253, 98, 253, 98, 112, 103, 112, 103, 235, 237, 112, 72, 112, 103, 1, 68, 1, 230, 2, 171, 1, 36, 117, 0, 150, 34, 98, 0, 245, 98, 245, 98, 245, 98, 245, 98, 245, 98, 245, 98, 245, 66, 228, 224, 245, 66, 237, 65, 245, 65, 253, 162, 245, 97, 245, 131, 253, 161, 245, 97, 245, 65, 245, 98, 245, 66, 245, 98, 245, 98, 245, 97, 245, 98, 228, 225, 245, 98, 245, 97, 245, 97, 245, 130, 253, 196, 245, 131, 253, 161, 245, 99, 245, 130, 245, 129, 237, 97, 245, 66, 245, 98, 245, 130, 253, 161, 253, 162, 245, 97, 245, 97, 237, 65, 245, 98, 237, 34, 236, 225, 236, 225, 237, 2, 245, 65, 245, 65, 237, 33, 237, 33, 245, 66, 245, 65, 24, 160, 66, 72, 115, 76, 107, 108, 107, 139, 107, 108, 74, 104, 16, 163, 237, 199, 237, 161, 245, 98, 245, 98, 245, 98, 245, 97, 245, 98, 237, 160, 245, 130, 72, 128, 204, 162, 254, 83, 196, 162, 72, 128, 72, 129, 1, 67, 3, 109, 2, 202, 1, 68, 158, 64, 97, 224, 245, 129, 245, 162, 245, 97, 245, 65, 237, 1, 237, 2, 245, 33, 245, 65, 245, 97, 253, 130, 253, 161, 253, 161, 245, 97, 245, 98, 245, 97, 245, 97, 245, 97, 245, 130, 245, 98, 245, 33, 237, 1, 245, 2, 237, 97, 245, 98, 245, 97, 245, 130, 245, 161, 245, 161, 245, 161, 253, 98, 236, 225, 204, 199, 222, 46, 237, 165, 237, 130, 245, 34, 245, 97, 245, 97, 245, 130, 253, 161, 253, 162, 245, 97, 245, 98, 236, 225, 236, 226, 236, 225, 237, 2, 245, 66, 237, 65, 245, 130, 245, 97, 237, 33, 237, 1, 245, 33, 237, 33, 40, 192, 41, 166, 41, 134, 49, 134, 41, 134, 24, 133, 229, 104, 245, 66, 245, 130, 245, 130, 245, 130, 245, 98, 245, 65, 237, 1, 245, 66, 72, 192, 204, 161, 254, 99, 254, 100, 246, 99, 254, 100, 204, 195, 72, 160, 2, 169, 1, 232, 1, 68, 117, 0, 97, 224, 253, 162, 253, 161, 245, 97, 245, 65, 237, 33, 236, 225, 237, 33, 253, 162, 253, 162, 253, 161, 253, 161, 245, 97, 245, 98, 237, 65, 245, 66, 245, 97, 245, 129, 253, 161, 245, 97, 245, 34, 236, 225, 236, 224, 204, 166, 222, 15, 229, 228, 253, 161, 253, 161, 253, 228, 253, 197, 245, 131, 220, 96, 179, 99, 196, 201, 236, 196, 245, 67, 245, 96, 237, 65, 245, 66, 245, 66, 245, 97, 245, 98, 245, 97, 245, 98, 245, 33, 236, 225, 237, 2, 245, 66, 245, 97, 245, 129, 253, 161, 245, 129, 245, 65, 237, 33, 244, 225, 237, 66, 253, 163, 40, 160, 41, 165, 33, 166, 24, 163, 221, 102, 237, 32, 245, 66, 245, 66, 253, 161, 253, 161, 245, 129, 245, 98, 245, 65, 229, 1, 64, 162, 254, 67, 246, 131, 246, 99, 246, 100, 246, 99, 246, 66, 72, 193, 1, 68, 1, 37, 116, 224, 158, 33, 97, 224, 253, 162, 245, 98, 245, 97, 245, 98, 245, 66, 237, 33, 237, 65, 253, 163, 253, 162, 245, 130, 245, 98, 245, 97, 245, 97, 245, 98, 245, 98, 245, 161, 245, 162, 245, 98, 245, 98, 245, 98, 245, 97, 220, 97, 171, 131, 204, 136, 228, 196, 253, 131, 245, 98, 253, 228, 245, 228, 253, 228, 228, 225, 220, 99, 228, 98, 236, 195, 245, 98, 245, 98, 245, 97, 245, 98, 245, 97, 245, 98, 245, 97, 245, 98, 245, 97, 245, 97, 245, 65, 245, 98, 245, 97, 253, 129, 253, 129, 245, 98, 245, 98, 245, 97, 245, 97, 245, 98, 237, 129, 245, 98, 65, 65, 33, 133, 41, 134, 41, 102, 229, 135, 237, 65, 245, 98, 253, 129, 245, 162, 245, 98, 245, 98, 245, 98, 245, 98, 72, 224, 129, 194, 204, 162, 129, 226, 212, 162, 129, 227, 72, 193, 254, 99, 204, 194, 72, 161, 158, 96, 117, 0, 199, 130, 97, 192, 245, 66, 245, 97, 253, 130, 253, 162, 253, 161, 237, 32, 237, 33, 237, 33, 245, 66, 245, 98, 237, 33, 236, 225, 237, 1, 245, 98, 245, 65, 245, 98, 245, 98, 245, 98, 245, 130, 253, 162, 253, 162, 236, 225, 220, 96, 228, 98, 236, 226, 253, 196, 229, 66, 228, 225, 237, 2, 253, 228, 253, 228, 237, 130, 245, 97, 253, 228, 245, 163, 253, 162, 245, 161, 245, 97, 245, 98, 253, 130, 245, 130, 253, 130, 245, 161, 245, 161, 245, 97, 245, 98, 245, 98, 245, 97, 245, 97, 245, 97, 245, 130, 253, 162, 245, 130, 245, 98, 237, 65, 65, 66, 66, 72, 140, 110, 140, 76, 107, 108, 41, 101, 221, 38, 245, 66, 237, 33, 245, 66, 245, 98, 253, 162, 253, 163, 253, 162, 72, 160, 204, 193, 113, 161, 204, 161, 129, 194, 204, 130, 64, 224, 212, 131, 244, 14, 72, 161, 150, 32, 158, 32, 199, 129, 98, 0, 237, 33, 245, 65, 245, 65, 245, 130, 245, 162, 253, 160, 237, 32, 237, 33, 237, 1, 245, 66, 237, 33, 237, 65, 237, 33, 236, 224, 245, 66, 245, 98, 245, 97, 245, 97, 245, 97, 245, 97, 253, 130, 253, 162, 253, 98, 237, 1, 236, 225, 253, 197, 245, 130, 253, 130, 236, 225, 236, 194, 245, 34, 253, 196, 245, 228, 246, 3, 253, 98, 245, 162, 253, 130, 245, 161, 237, 129, 245, 130, 245, 98, 245, 97, 253, 130, 253, 161, 253, 131, 245, 98, 245, 98, 245, 98, 253, 131, 253, 130, 245, 98, 245, 98, 245, 98, 245, 98, 245, 97, 57, 130, 107, 108, 140, 78, 181, 241, 148, 45, 41, 165, 220, 231, 237, 34, 245, 65, 237, 1, 237, 66, 245, 98, 253, 162, 112, 103, 64, 160, 246, 67, 246, 36, 254, 100, 254, 99, 204, 131, 80, 160, 104, 103, 254, 115, 104, 72, 158, 32, 199, 162, 125, 0, 97, 224, 245, 98, 245, 98, 245, 65, 237, 33, 245, 66, 245, 98, 237, 32, 245, 66, 245, 97, 237, 65, 237, 64, 237, 33, 237, 33, 245, 66, 245, 97, 245, 129, 253, 161, 253, 192, 245, 129, 245, 98, 245, 66, 245, 98, 245, 97, 245, 98, 245, 98, 245, 229, 245, 98, 204, 166, 214, 47, 238, 5, 246, 4, 253, 163, 253, 160, 253, 161, 237, 97, 245, 98, 245, 98, 245, 66, 253, 98, 237, 97, 245, 98, 245, 66, 245, 66, 245, 97, 245, 98, 245, 98, 245, 33, 236, 225, 237, 1, 245, 66, 245, 97, 245, 66, 245, 98, 237, 34, 179, 192, 57, 66, 74, 73, 107, 109, 140, 78, 140, 75, 57, 101, 221, 5, 236, 226, 245, 2, 245, 97, 245, 98, 245, 65, 237, 32, 112, 39, 72, 192, 196, 193, 246, 99, 246, 100, 246, 132, 204, 162, 112, 103, 246, 83, 254, 116, 120, 71, 158, 64, 150, 64, 199, 161, 97, 224, 245, 98, 245, 97, 245, 65, 245, 33, 236, 226, 245, 66, 245, 65, 245, 131, 245, 161, 245, 192, 245, 162, 237, 65, 245, 65, 245, 130, 245, 161, 253, 160, 253, 161, 245, 98, 245, 98, 245, 33, 237, 1, 245, 66, 245, 98, 245, 162, 245, 161, 245, 128, 220, 97, 179, 68, 196, 201, 228, 227, 245, 194, 253, 160, 253, 131, 245, 228, 245, 130, 253, 66, 205, 160, 173, 224, 213, 128, 189, 192, 213, 97, 236, 225, 237, 1, 245, 66, 245, 97, 237, 33, 236, 225, 245, 2, 245, 66, 245, 98, 245, 97, 237, 33, 237, 1, 245, 34, 163, 160, 106, 32, 57, 100, 66, 72, 74, 72, 41, 167, 147, 132, 163, 96, 236, 227, 245, 34, 245, 98, 245, 98, 245, 64, 237, 1, 236, 225, 72, 192, 64, 160, 64, 193, 72, 192, 64, 160, 64, 160, 112, 71, 254, 82, 252, 45, 112, 72, 117, 0, 158, 65, 199, 129, 105, 224, 237, 65, 237, 32, 245, 66, 236, 224, 245, 34, 237, 65, 245, 161, 253, 160, 253, 162, 237, 65, 245, 98, 245, 98, 237, 65, 237, 65, 245, 66, 245, 97, 245, 66, 237, 33, 245, 98, 236, 224, 245, 34, 245, 66, 245, 161, 253, 192, 253, 130, 245, 98, 236, 225, 220, 97, 220, 66, 236, 194, 245, 130, 253, 228, 245, 129, 237, 33, 245, 66, 245, 162, 237, 130, 221, 162, 173, 192, 197, 129, 173, 160, 221, 97, 237, 98, 228, 160, 228, 65, 245, 34, 237, 129, 245, 97, 245, 65, 237, 33, 245, 65, 229, 1, 237, 33, 245, 97, 171, 128, 98, 0, 24, 161, 41, 134, 41, 103, 16, 163, 90, 3, 163, 130, 245, 68, 245, 66, 237, 66, 237, 33, 245, 65, 236, 225, 237, 1, 0, 160, 2, 225, 2, 64, 2, 97, 2, 225, 2, 64, 10, 32, 112, 71, 112, 71, 117, 0, 142, 64, 158, 65, 158, 32, 89, 224, 253, 161, 237, 65, 245, 97, 245, 98, 245, 66, 237, 65, 253, 131, 245, 97, 237, 66, 237, 65, 236, 225, 237, 33, 237, 65, 237, 65, 245, 65, 245, 129, 253, 130, 245, 98, 245, 130, 245, 98, 245, 66, 245, 97, 253, 131, 245, 98, 245, 97, 245, 98, 236, 224, 253, 130, 245, 98, 253, 130, 253, 228, 245, 193, 253, 162, 245, 130, 245, 130, 253, 65, 237, 130, 229, 129, 221, 193, 140, 96, 165, 224, 213, 193, 189, 192, 220, 128, 254, 67, 244, 160, 245, 34, 245, 161, 253, 162, 245, 66, 237, 98, 245, 34, 245, 97, 245, 66, 171, 96, 106, 32, 24, 193, 41, 135, 66, 105, 16, 163, 98, 68, 163, 194, 245, 132, 245, 192, 253, 162, 237, 65, 245, 65, 245, 65, 245, 66, 0, 160, 2, 225, 2, 65, 1, 97, 2, 225, 2, 65, 2, 96, 8, 128, 150, 32, 117, 32, 166, 34, 199, 128, 158, 96, 57, 0, 245, 97, 237, 33, 245, 97, 245, 130, 245, 130, 237, 65, 237, 33, 245, 33, 237, 34, 245, 33, 245, 33, 245, 65, 237, 65, 245, 130, 253, 161, 253, 162, 245, 97, 245, 97, 245, 97, 253, 163, 253, 163, 245, 67, 245, 129, 237, 1, 245, 1, 245, 66, 253, 98, 245, 130, 245, 130, 245, 130, 245, 161, 253, 161, 245, 131, 245, 98, 245, 66, 245, 130, 164, 64, 99, 32, 221, 196, 140, 96, 124, 160, 214, 162, 213, 160, 106, 224, 220, 160, 165, 64, 166, 0, 237, 162, 245, 96, 206, 64, 213, 224, 237, 194, 221, 130, 198, 96, 155, 224, 106, 96, 16, 129, 66, 73, 66, 72, 24, 196, 98, 34, 171, 162, 245, 100, 245, 162, 245, 97, 237, 33, 245, 97, 253, 162, 245, 130, 0, 161, 2, 97, 2, 33, 0, 160, 3, 1, 2, 96, 2, 64, 0, 192, 142, 64, 150, 64, 207, 98, 150, 64, 158, 32, 57, 0, 237, 65, 245, 65, 253, 130, 253, 162, 245, 98, 237, 33, 236, 194, 236, 193, 236, 225, 236, 226, 237, 2, 245, 66, 245, 130, 253, 161, 245, 96, 245, 97, 245, 98, 245, 98, 245, 130, 245, 131, 229, 1, 204, 231, 214, 47, 238, 78, 221, 4, 236, 194, 237, 2, 245, 98, 245, 129, 253, 160, 245, 129, 245, 98, 245, 129, 245, 129, 253, 130, 253, 195, 229, 98, 221, 131, 107, 0, 116, 64, 83, 32, 157, 224, 99, 64, 197, 132, 189, 161, 222, 1, 116, 192, 182, 34, 213, 226, 237, 226, 132, 128, 221, 131, 198, 97, 213, 161, 172, 0, 198, 100, 24, 224, 66, 105, 107, 108, 24, 131, 147, 229, 245, 194, 245, 64, 245, 34, 237, 65, 245, 65, 253, 130, 253, 162, 245, 97, 237, 33, 40, 227, 147, 43, 81, 198, 2, 96, 2, 225, 2, 64, 0, 160, 191, 193, 207, 129, 150, 64, 150, 65, 150, 64, 64, 224, 237, 34, 237, 1, 245, 65, 245, 97, 245, 66, 245, 98, 245, 98, 245, 65, 236, 225, 245, 2, 245, 66, 245, 65, 237, 65, 237, 65, 245, 98, 228, 224, 245, 66, 237, 33, 245, 98, 245, 129, 220, 96, 179, 67, 196, 201, 196, 200, 213, 3, 237, 1, 245, 98, 245, 97, 245, 129, 245, 97, 245, 130, 245, 98, 245, 129, 245, 162, 253, 98, 245, 129, 245, 66, 245, 128, 245, 228, 91, 0, 34, 0, 157, 227, 75, 64, 132, 64, 206, 2, 124, 32, 100, 128, 117, 64, 182, 130, 221, 194, 141, 128, 116, 96, 67, 0, 221, 132, 206, 33, 25, 128, 16, 162, 74, 106, 107, 140, 24, 195, 16, 163, 237, 134, 237, 32, 244, 193, 245, 1, 236, 226, 245, 98, 245, 99, 138, 227, 32, 227, 40, 164, 147, 75, 73, 231, 40, 163, 147, 76, 81, 198, 40, 196, 99, 67, 191, 130, 158, 32, 150, 96, 108, 224, 83, 128, 245, 130, 245, 98, 245, 97, 245, 65, 245, 33, 236, 225, 237, 33, 237, 65, 237, 65, 245, 98, 253, 161, 253, 161, 253, 161, 253, 162, 245, 98, 245, 98, 245, 65, 245, 65, 245, 98, 245, 97, 228, 193, 228, 66, 228, 67, 236, 194, 245, 66, 245, 97, 253, 130, 253, 161, 245, 192, 253, 130, 245, 98, 245, 98, 245, 130, 253, 98, 245, 129, 236, 226, 245, 34, 253, 66, 65, 128, 124, 65, 42, 32, 124, 162, 42, 0, 157, 229, 91, 64, 174, 131, 59, 64, 59, 64, 125, 160, 116, 128, 59, 96, 174, 228, 59, 128, 100, 32, 134, 1, 66, 131, 8, 163, 24, 195, 16, 131, 24, 196, 74, 137, 16, 163, 221, 136, 253, 163, 245, 162, 245, 65, 237, 130, 139, 2, 40, 196, 179, 206, 89, 198, 81, 199, 40, 164, 40, 163, 139, 76, 81, 198, 40, 195, 66, 164, 83, 129, 158, 33, 117, 0, 158, 65, 117, 0, 237, 65, 236, 224, 245, 66, 245, 34, 236, 225, 237, 1, 245, 66, 237, 65, 245, 66, 245, 131, 253, 161, 253, 161, 253, 161, 245, 98, 245, 97, 245, 65, 237, 33, 236, 192, 245, 66, 237, 33, 236, 225, 237, 1, 245, 98, 245, 97, 253, 97, 253, 163, 245, 161, 253, 161, 253, 162, 245, 130, 245, 98, 245, 130, 245, 130, 253, 130, 237, 65, 244, 193, 245, 97, 237, 97, 197, 99, 50, 0, 116, 132, 75, 0, 124, 130, 83, 64, 100, 128, 59, 0, 174, 197, 59, 64, 134, 35, 43, 96, 51, 96, 133, 193, 92, 128, 196, 227, 141, 192, 92, 160, 49, 195, 107, 141, 140, 110, 49, 164, 16, 164, 24, 195, 229, 102, 237, 97, 237, 65, 236, 225, 237, 65, 123, 3, 40, 196, 40, 163, 32, 197, 32, 195, 179, 238, 139, 77, 81, 166, 73, 230, 32, 196, 66, 225, 83, 99, 158, 65, 117, 0, 199, 99, 158, 64, 245, 98, 245, 98, 245, 97, 245, 97, 245, 130, 253, 162, 245, 98, 245, 33, 236, 225, 237, 2, 245, 65, 245, 97, 245, 98, 245, 97, 245, 160, 245, 161, 245, 98, 245, 98, 245, 97, 245, 98, 245, 130, 245, 162, 253, 130, 245, 66, 237, 97, 253, 162, 245, 97, 245, 98, 245, 97, 245, 97, 245, 98, 245, 98, 237, 65, 245, 98, 237, 65, 237, 32, 122, 224, 91, 32, 33, 32, 33, 192, 116, 96, 42, 0, 42, 96, 75, 32, 59, 0, 58, 224, 134, 34, 59, 128, 92, 128, 158, 163, 67, 64, 84, 129, 196, 225, 254, 131, 228, 224, 24, 224, 107, 108, 66, 40, 66, 72, 107, 107, 16, 195, 16, 163, 229, 197, 253, 161, 245, 98, 245, 98, 237, 33, 139, 4, 131, 34, 131, 66, 139, 35, 130, 226, 32, 196, 40, 196, 32, 196, 32, 194, 40, 196, 58, 194, 108, 3, 158, 64, 116, 224, 150, 32, 199, 160, 245, 67, 245, 98, 245, 98, 245, 98, 245, 98, 245, 98, 245, 66, 228, 224, 245, 66, 237, 65, 245, 65, 253, 162, 245, 97, 245, 131, 253, 161, 245, 97, 245, 65, 245, 98, 245, 66, 245, 65, 245, 98, 245, 130, 253, 161, 253, 162, 245, 97, 245, 97, 237, 65, 245, 98, 237, 34, 236, 225, 236, 225, 237, 2, 245, 65, 245, 65, 237, 34, 237, 33, 229, 68, 213, 164, 93, 69, 94, 104, 35, 64, 9, 96, 117, 167, 102, 134, 102, 135, 85, 33, 43, 32, 59, 96, 18, 32, 67, 96, 174, 198, 59, 128, 115, 224, 228, 160, 244, 0, 58, 0, 133, 102, 33, 194, 57, 200, 66, 105, 24, 163, 16, 228, 237, 164, 237, 130, 245, 66, 245, 99, 245, 65, 245, 132, 139, 2, 131, 2, 139, 4, 130, 193, 130, 194, 130, 194, 138, 195, 66, 3, 58, 227, 108, 35, 158, 32, 158, 64, 158, 64, 158, 96, 199, 129, 245, 97, 253, 162, 245, 97, 245, 65, 237, 1, 237, 2, 245, 33, 245, 65, 245, 97, 253, 130, 253, 161, 253, 161, 245, 97, 245, 98, 245, 97, 245, 97, 245, 97, 253, 130, 245, 97, 237, 33, 245, 98, 245, 97, 245, 130, 253, 161, 253, 162, 245, 97, 245, 98, 236, 225, 236, 226, 236, 225, 237, 2, 245, 66, 237, 65, 245, 162, 245, 65, 237, 65, 228, 225, 237, 34, 34, 64, 51, 128, 110, 136, 1, 160, 102, 39, 44, 0, 76, 160, 26, 96, 100, 163, 51, 32, 18, 64, 43, 33, 125, 160, 51, 160, 133, 228, 75, 64, 49, 224, 133, 132, 34, 32, 41, 133, 58, 40, 41, 70, 16, 163, 229, 71, 245, 66, 245, 65, 237, 97, 253, 163, 245, 65, 245, 65, 236, 225, 237, 33, 122, 227, 130, 195, 74, 162, 83, 130, 116, 67, 116, 64, 158, 32, 199, 128, 158, 64, 158, 64, 199, 130, 117, 0, 150, 33];