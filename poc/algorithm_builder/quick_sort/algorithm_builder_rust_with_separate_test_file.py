
"""
In this POC, we demonstrate how to generate a complete Rust program
and its corresponding build script using OpenAI's API.
Though the main program generation runs successfully, we need to ensure
that the generated test cases are also valid and cover all edge cases.
But, there is some difficulty to create those test cases in
separate files, build them, or run them successfully.

An end-to-end test program for generating a complete
Rust program and build script.
"""
import asyncio
import os
# import json

# from openai import AzureOpenAI
# from openai import AsyncAzureOpenAI
from openai import ChatCompletion

# from utility_debugging.utility_debugging import DebuggingUtility
from utility_io.io_utility import IoUtility
from utility_openai.utility_openai_azure_foundry import OpenaiAzureFoundryUtility

async def main_test(write_to_file: bool = True):
    """
    An end-to-end test program for generating a complete
    Rsut program and build script.
    """
    # ------------------------------------------------------------------------
    # ------------------------------------------------------------------------
    generative_model_configuration = \
        OpenaiAzureFoundryUtility.get_generative_model_configuration()
    # print(generative_model_configuration)
    generative_model_parameters = \
        OpenaiAzureFoundryUtility.get_generative_model_parameters()
    bearer_token_provider = \
        OpenaiAzureFoundryUtility.create_bearer_token_provider(\
            generative_model_configuration['token_provider_endpoint'])
    generative_ai_client = \
        OpenaiAzureFoundryUtility.create_generative_ai_client(\
            generative_model_configuration['endpoint'],
            bearer_token_provider,
            generative_model_configuration['api_version'])
    # ------------------------------------------------------------------------
    # ------------------------------------------------------------------------
    prompt = 'Write a program implementing ' \
        'the quick sort algorithm in Rust. ' \
        'This program should be efficient, ' \
        'well-documented, and can be compiled.'
    generative_ai_completion: ChatCompletion = \
        OpenaiAzureFoundryUtility.\
            request_generative_ai_completion_using_single_prompt(
                generative_ai_client,
                generative_model_configuration,
                generative_model_parameters,
                prompt)
    # print('---- DEBUGGING-generative_ai_completion ----', \
    #       generative_ai_completion)
    generated_program_payload = \
        generative_ai_completion.choices[0].message.content
    output_file_full_path = os.path.join(
        'generated',
        'poc_algorithm_builder_rust_quick_sort_' \
        'generated_program_payload.txt')
    if write_to_file:
        await IoUtility.create_file_async(
            content=generated_program_payload,
            content_permanent_file_name=\
                output_file_full_path,
            content_output_directory='.'
        )
    # ---- NOTE-TODO ---- response_payload = \
    # ---- NOTE-TODO ----     OpenaiAzureFoundryUtility.\
    # ---- NOTE-TODO ----         get_chat_response(generative_ai_completion)
    # ---- NOTE-TODO ---- print('---- DEBUGGING-response_payload ----', \
    # ---- NOTE-TODO ----     response_payload)
    # ---- NOTE-TODO ---- generated_program = \
    # ---- NOTE-TODO ----     generative_ai_completion\
    # ---- NOTE-TODO ----         ['choices'][0]['message']['content']
    # ---- NOTE-TODO ---- print('---- DEBUGGING-generated_program ----', \
    # ---- NOTE-TODO ----     generated_program)
    # ------------------------------------------------------------------------
    prompt = 'Split the following text into ' \
        'parts delimited by two lines. ' \
        'The first line starts with "```rust" ' \
        'and the second starts with "```". ' \
        'Only output the second part ' \
        'without the delimiter lines. ' \
        'In the comment, call the program ' \
        '"poc_algorithm_builder_rust_quick_sort_' \
        'generated_program_payload.rs". ' \
        f'Below is the text:\n\n{generated_program_payload}'
    generative_ai_completion: ChatCompletion = \
        OpenaiAzureFoundryUtility.\
            request_generative_ai_completion_using_single_prompt(
            generative_ai_client,
            generative_model_configuration,
            generative_model_parameters,
            prompt)
    # print('---- DEBUGGING-generative_ai_completion ----', \
    #       generative_ai_completion)
    generated_program_payload = \
        generative_ai_completion.choices[0].message.content
    output_file_full_path = os.path.join(
        'generated',
        'poc_algorithm_builder_rust_quick_sort_' \
        'generated_program_payload.rs')
    if write_to_file:
        await IoUtility.create_file_async(
            content=generated_program_payload,
            content_permanent_file_name=\
                output_file_full_path,
            content_output_directory='.'
        )
    # ------------------------------------------------------------------------
    # ------------------------------------------------------------------------
    prompt = 'Generate unit tests for the Rust program below.' \
        'The tests should cover every functions in the program.' \
        'The tests should also be compilable and include all necessary ' \
        'imports.' \
        'In the comment, call the program ' \
        '"poc_algorithm_builder_rust_quick_sort_' \
        'generated_program_payload_tests.rs". ' \
        f'Below is the text:\n\n{generated_program_payload}'
    generative_ai_completion: ChatCompletion = \
        OpenaiAzureFoundryUtility.\
            request_generative_ai_completion_using_single_prompt(
            generative_ai_client,
            generative_model_configuration,
            generative_model_parameters,
            prompt)
    # print('---- DEBUGGING-generative_ai_completion ----', \
    #       generative_ai_completion)
    generated_program_payload = \
        generative_ai_completion.choices[0].message.content
    output_file_full_path = os.path.join(
        'generated',
        'poc_algorithm_builder_rust_quick_sort_' \
        'generated_program_payload_tests.txt')
    if write_to_file:
        await IoUtility.create_file_async(
            content=generated_program_payload,
            content_permanent_file_name=\
                output_file_full_path,
            content_output_directory='.'
        )
    # ------------------------------------------------------------------------
    prompt = 'Split the following text into ' \
        'parts delimited by two lines. ' \
        'The first line starts with "```rust" ' \
        'and the second starts with "```". ' \
        'Only output the first part ' \
        'without the delimiter lines. ' \
        'In the comment, call the program ' \
        '"poc_algorithm_builder_rust_quick_sort_' \
        'generated_program_payload_tests.rs". ' \
        f'Below is the text:\n\n{generated_program_payload}'
    generative_ai_completion: ChatCompletion = \
        OpenaiAzureFoundryUtility.\
            request_generative_ai_completion_using_single_prompt(
            generative_ai_client,
            generative_model_configuration,
            generative_model_parameters,
            prompt)
    # print('---- DEBUGGING-generative_ai_completion ----', \
    #       generative_ai_completion)
    generated_program_payload = \
        generative_ai_completion.choices[0].message.content
    output_file_full_path = os.path.join(
        'generated',
        'poc_algorithm_builder_rust_quick_sort_' \
        'generated_program_payload_tests.rs')
    if write_to_file:
        await IoUtility.create_file_async(
            content=generated_program_payload,
            content_permanent_file_name=\
                output_file_full_path,
            content_output_directory='.'
        )
    # ------------------------------------------------------------------------
    # ------------------------------------------------------------------------
    prompt = 'Create a Windows command-line ' \
        'script to initialize a Rust project, ' \
        'which can build and run a rust program ' \
        'called ' \
        '"poc_algorithm_builder_rust_quick_sort_generated_program' \
        '_payload.rs". ' \
        ' and its test program ' \
        '"poc_algorithm_builder_rust_quick_sort_generated_program' \
        '_payload_tests.rs". ' \
        'The script should include commands to create the necessary ' \
        'directory structure, src and src/tests, copy the generated Rust ' \
        'files into place, ' \
        'and run the tests using Cargo.' \
        'Only output the commands without any explanations, ' \
        'additional text, or delimiters.'
    generative_ai_completion: ChatCompletion = \
        OpenaiAzureFoundryUtility.\
            request_generative_ai_completion_using_single_prompt(
                generative_ai_client,
                generative_model_configuration,
                generative_model_parameters,
                prompt)
    # print('---- DEBUGGING-generative_ai_completion ----', \
    #       generative_ai_completion)
    generated_program_payload = \
        generative_ai_completion.choices[0].message.content
    output_file_full_path = os.path.join(
        'generated',
        'poc_algorithm_builder_rust_quick_sort_' \
        'generated_program_payload_extracted_project_script.cmd')
    if write_to_file:
        await IoUtility.create_file_async(
            content=generated_program_payload,
            content_permanent_file_name=\
                output_file_full_path,
            content_output_directory='.'
        )
    # ------------------------------------------------------------------------
    # ------------------------------------------------------------------------

if __name__ == "__main__":
    asyncio.run(main_test())
