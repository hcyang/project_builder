
"""
This module provides a common utility for debugging and tracing.
"""

import sys

class DebuggingUtility:
    """
    This class provides a common interface for debugging and tracing.
    It can be extended for instrumentation.
    """
    @staticmethod
    def info(message):
        """
        Produce an info message.
        """
        print(f'INFO: {message}')
    @staticmethod
    def debug(message):
        """
        Produce a debug message.
        """
        print(f'DEBUG: {message}')
    @staticmethod
    def error(message):
        """
        Produce an error message.
        """
        print(f'ERROR: {message}')

    @staticmethod
    def dump_sys_path():
        """
        Dump sys path.
        """
        DebuggingUtility.info(f'sys.path: {sys.path}')

if __name__ == "__main__":
    # DebuggingUtility.info('Hello, World')
    DebuggingUtility.dump_sys_path()
