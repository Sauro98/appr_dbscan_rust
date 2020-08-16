#include<iostream>
#include<fstream>
#include<stddef.h>

typedef struct {
    uint8_t cntrl_1;
    uint8_t cntrl_2;
    uint32_t file_size;
    uint32_t unused;
    uint32_t offset;
} bmp_header_t;

void print_bmp_header(bmp_header_t& header){
    std::cout<<"----------BITMAP HEADER ----------"<<std::endl;
    std::cout<<header.cntrl_1<<header.cntrl_2<<std::endl; 
    std::cout<<"file_size: "<<header.file_size<<std::endl;
    std::cout<<"data_offset: "<<header.offset<<std::endl;
}

typedef struct {
    uint32_t header_size;
    uint32_t width;
    uint32_t height;
    uint16_t planes;
    uint16_t bits_per_pixel;
    uint32_t compression_method;
    uint32_t image_size;
    uint32_t h_res;
    uint32_t w_res;
    uint32_t colors_count;
    uint32_t important_colors;
} dib_header_t;

void print_dib_header(dib_header_t& header){
    std::cout<<"----------DIB HEADER ----------"<<std::endl;
    std::cout<<"header_size: "<<header.header_size<<std::endl;
    std::cout<<"width: "<<header.width<<std::endl;
    std::cout<<"height: "<<header.height<<std::endl;
    std::cout<<"planes: "<<header.planes<<std::endl;
    std::cout<<"bits_per_pixel: "<<header.bits_per_pixel<<std::endl;
    std::cout<<"compression_method: "<<header.compression_method<<std::endl;
    std::cout<<"image_size: "<<header.image_size<<std::endl;
    std::cout<<"h_res: "<<header.h_res<<std::endl;
    std::cout<<"w_res: "<<header.w_res<<std::endl;
    std::cout<<"colors_count: "<<header.colors_count<<std::endl;
    std::cout<<"important_colors: "<<header.important_colors<<std::endl;
}

uint32_t read_uint(std::ifstream& ifs){
    uint32_t val = 0;
    for(int i = 0; i < 4; i++){
        val |= (((uint32_t)ifs.get())<< (8 * i));
    } 
    return val;
}

uint16_t read_ushort(std::ifstream& ifs){
    uint16_t val = 0;
    for(int i = 0; i < 2; i++){
        val |= (((uint16_t)ifs.get())<< (8 * i));
    } 
    return val;
}

bmp_header_t read_bmp_header(std::ifstream& ifs){
    bmp_header_t header;
    header.cntrl_1 = ifs.get();
    header.cntrl_2 = ifs.get();
    header.file_size = read_uint(ifs);
    header.unused = read_uint(ifs);
    header.offset = read_uint(ifs);
    return header;
}

dib_header_t read_dib_header(std::ifstream& ifs){
    dib_header_t header;
    header.header_size = read_uint(ifs);
    header.width = read_uint(ifs);
    header.height = read_uint(ifs);
    header.planes = read_ushort(ifs);
    header.bits_per_pixel = read_ushort(ifs);
    header.compression_method = read_uint(ifs);
    header.image_size = read_uint(ifs);
    header.h_res = read_uint(ifs);
    header.w_res = read_uint(ifs);
    header.colors_count = read_uint(ifs);
    header.important_colors = read_uint(ifs);
    return header;
}

int main(int argc, char** argv){
    std::ifstream ifs (argv[1], std::ifstream::in);
    if(!ifs.is_open()){
        std::cout<<"Failed to open file"<<std::endl;
        return 0;
    }

    // get length of file:
    ifs.seekg (0, ifs.end);
    int length = ifs.tellg();
    ifs.seekg (0, ifs.beg);
    std::cout<<"stream file length:"<<length<<std::endl;

    bmp_header_t bmp_header = read_bmp_header(ifs);
    print_bmp_header(bmp_header);

    dib_header_t dib_header = read_dib_header(ifs);
    print_dib_header(dib_header);
    std::cout<<"--------------------"<<std::endl;
    
    unsigned int curr_pos = ifs.tellg();
    std::cout<<"Beginning to read bitmap data at pos: "<<curr_pos<<std::endl;

    std::ofstream ofs ("test_bitmap.txt",std::ios_base::binary);
    char * buffer  = new char [3 * dib_header.width];
    unsigned int bytes_count = 14 + dib_header.header_size;
    unsigned int chars_read = bytes_count;
    for(int r = 0; r < dib_header.height; r++){
        ifs.read(buffer, 3 * dib_header.width);
        for(int c = 0; c < dib_header.width; c++){
                uint32_t BLUE = (unsigned char)buffer[(3 * c) + 0];
                uint32_t GREEN = (unsigned char)buffer[(3 * c) + 1];
                uint32_t RED = (unsigned char)buffer[(3 * c) + 2];
                ofs<<r<<" "<<c<<" "<<BLUE<<" "/*<<GREEN<<" "<<RED<<" "*/<<std::endl;
                 
        }
        bytes_count+=  3 * dib_header.width;
        chars_read += ifs.gcount();
        for(int p = 0; p < (dib_header.width % 4); p++){
            std::cout<<"Reading padding"<<std::endl;
            ifs.get();
            bytes_count ++;
            chars_read += ifs.gcount(); 
        }
        if(ifs.eof()){
            std::cout<<"!!!!Failure in reading file(eof reached)!!!!"<<std::endl;
            break;
        }
        if(ifs.bad()){
            std::cout<<"!!!!Failure in reading file(badbit)!!!!"<<std::endl;
            break;
        }
        if(ifs.fail()){
            std::cout<<"!!!!Failure in reading file(failbit)!!!!"<<std::endl;
            break;
        }
    }
    delete[] buffer;
    std::cout<<"Read "<<bytes_count<<" bytes"<<std::endl;
    

    ifs.close();
    ofs.close();

    return 0;
}