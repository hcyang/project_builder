"""
This module contains I/O utility functions.
"""

import os
import aiofiles

# from utility_debugging.utility_debugging import DebuggingUtility

class IoUtility:
    """
    I/O utility functions.
    """
    @staticmethod
    async def create_file_async(
        content: str,
        content_permanent_file_name: str = '',
        content_output_directory: str = '.') -> str:
        """
        Create a file asynchronously.
        """
        content_permanent_file_path = \
            os.path.join(
                content_output_directory,
                content_permanent_file_name
            )
        async with aiofiles.open(
            content_permanent_file_path,
            'w') as file:
            await file.write(content)
        return content_permanent_file_path
    @staticmethod
    def create_file(
        content: str,
        content_permanent_file_name: str = '',
        content_output_directory: str = '.') -> str:
        """
        Create a file.
        """
        content_permanent_file_path = \
            os.path.join(
                content_output_directory,
                content_permanent_file_name
            )
        with open(
            content_permanent_file_path,
            'w',
            encoding='utf-8') as file:
            file.write(content)
        return content_permanent_file_path
