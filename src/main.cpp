
#define COMPILE_SCRIPT_PREFIX string("\
#include <stdlib.h>\n\
#include <string>\n\
int main(){return system(std::string(\"")
#define COMPILE_SCRIPT_SUFFIX string("\").c_str());}")

#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <regex>

using namespace std;


string readFile(string file)
{
    ifstream inputFile(file);
    string fC;

    if (!inputFile.is_open())
    {
        cout << "Failed to open file: " << file << endl;
        exit(EXIT_FAILURE);
    }

    string line;

    while (getline(inputFile, line))
    {
        fC += line + "\n";
    }

    inputFile.close();

    return fC;
}

void createTempCPPFile(string file, string script)
{
    ofstream outputFile(file);

    if (!outputFile.is_open())
    {
        cout << "Failed to open file: " << file << endl;
        exit(EXIT_FAILURE);
    }

    // this solution is trash but I'm too stupid for replace
    outputFile << COMPILE_SCRIPT_PREFIX << script << COMPILE_SCRIPT_SUFFIX;
}

string escapeScript(string &script)
{
    replace(script.begin(), script.end(), '\n', '&');
}

vector<string> split(string str, char delimiter)
{
    vector<string> result;
    stringstream strs(str);
    string token;

    while (getline(strs, token, delimiter))
    {
        result.push_back(token);
    }

    return result;
}

string stripExtension(string filename) {
    size_t lastdot = filename.find_last_of(".");
    
    if(lastdot == string::npos) return filename;

    return filename.substr(0, lastdot);
}

string stripPath(string path) {
    return split(path, '/').back();
}



void compile(string script, string out)
{
    string tmpFile = out + string("__tmp.cpp");

    // escape \n
    escapeScript(script);

    // create cpp file which runs the shell commands
    createTempCPPFile(tmpFile, script);

    string cmd = string("g++ ") + tmpFile + string(" -o ") + out;

    // compile the temp file
    system(cmd.c_str());

    // delete temp file
    remove(tmpFile.c_str());
}


int main(int argc, char *argv[])
{
    if (argc < 2)
    {
        cout << "No input files were specified." << endl;
        return EXIT_FAILURE;
    }

    string script = readFile(argv[1]);

    cout << "Compiling..." << endl;

    compile(script, stripPath(stripExtension(argv[1])));

    return EXIT_SUCCESS;
}
