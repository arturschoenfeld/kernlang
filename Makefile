CC := gcc
CFLAGS := -g -Wall -Wextra -Iinclude -fno-builtin
SRC_DIR := src
INC_DIR := include
OBJ_DIR := build

SRCS := $(wildcard $(SRC_DIR)/*.c)

OBJS := $(SRCS:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)

TARGET := bin/kern

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c | $(OBJ_DIR)
	$(CC) $(CFLAGS) -MMD -MP -c $< -o $@

$(OBJ_DIR):
	mkdir -p $(OBJ_DIR)

-include $(OBJS:.o=.d)

clean:
	rm -rf $(OBJ_DIR) $(TARGET)

.PHONY: all clean