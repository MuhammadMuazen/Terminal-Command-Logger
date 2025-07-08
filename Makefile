CC = gcc
CFLAGS = -Wall -Wextra -I./lib/cJSON
LDFLAGS = -lm  # cJSON may need math library on some systems

# Source files
SRC_DIR = src
SRCS = $(wildcard $(SRC_DIR)/*.c)

# cJSON source
CJSON_SRC = lib/cJSON/cJSON.c

# Object files
OBJS = $(SRCS:.c=.o) $(CJSON_SRC:.c=.o)

# Executable name
TARGET = tcl

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# Run the program
run: $(TARGET)
	./$(TARGET)

clean:
	rm -f $(OBJS) $(TARGET)

.PHONY: all clean