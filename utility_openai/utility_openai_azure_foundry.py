"""
This module provides a common utility for accessing Azure OpenAI models.
"""

import asyncio
import os
import json

from openai import AzureOpenAI
from openai import AsyncAzureOpenAI

from azure.identity import DefaultAzureCredential, get_bearer_token_provider

from utility_debugging.utility_debugging import DebuggingUtility
from utility_python.utility_python_code_runner import PythonCodeRunnerUtility

class OpenaiAzureFoundryUtility:
    """
    This class provides a common utility for accessing Azure OpenAI models.
    """
    @staticmethod
    def get_generative_model_configuration():
        """
        Retrieve model configuration from environment variables.
        """
        endpoint = os.getenv('ENDPOINT_URL')
        if not endpoint:
            raise ValueError(
                'The environment variable ENDPOINT_URL has not been set yet.')
        deployment = os.getenv('DEPLOYMENT_NAME')
        if not deployment:
            raise ValueError(
                'The environment variable DEPLOYMENT_NAME has not been set yet.')
        token_provider_endpoint = os.getenv('TOKEN_PROVIDER_ENDPOINT_URL')
        if not token_provider_endpoint:
            raise ValueError(
                'The environment variable TOKEN_PROVIDER_ENDPOINT_URL has not been set yet.')
        api_version = os.getenv('API_VERSION')
        if not api_version:
            raise ValueError(
                'The environment variable API_VERSION has not been set yet.')

        generative_model_configuration = {
            'endpoint': endpoint,
            'deployment': deployment,
            'token_provider_endpoint': token_provider_endpoint,
            'api_version': api_version
        }
        return generative_model_configuration

    @staticmethod
    def get_generative_model_parameters():
        """
        Set up a common set of model parameters.
        """
        azure_openai_client_parameter_max_tokens=9600
        azure_openai_client_parameter_temperature=0.7
        azure_openai_client_parameter_top_p=0.95
        azure_openai_client_parameter_frequency_penalty=0
        azure_openai_client_parameter_presence_penalty=0
        azure_openai_client_parameter_stop=None
        azure_openai_client_parameter_stream=False
        generative_model_parameters = {
            'azure_openai_client_parameter_max_tokens': \
                azure_openai_client_parameter_max_tokens,
            'azure_openai_client_parameter_temperature': \
                azure_openai_client_parameter_temperature,
            'azure_openai_client_parameter_top_p': \
                azure_openai_client_parameter_top_p,
            'azure_openai_client_parameter_frequency_penalty': \
                azure_openai_client_parameter_frequency_penalty,
            'azure_openai_client_parameter_presence_penalty': \
                azure_openai_client_parameter_presence_penalty,
            'azure_openai_client_parameter_stop': \
                azure_openai_client_parameter_stop,
            'azure_openai_client_parameter_stream': \
                azure_openai_client_parameter_stream
        }
        return generative_model_parameters

    @staticmethod
    def create_bearer_token_provider(token_provider_endpoint: str):
        """
        Create a bearer token provider.
        """
        # Initialize Azure OpenAI Service client with Entra ID authentication
        bearer_token_provider = get_bearer_token_provider(
            DefaultAzureCredential(),
            token_provider_endpoint
        )
        return bearer_token_provider

    @staticmethod
    def create_generative_ai_client(endpoint: str, bearer_token_provider, api_version: str):
        """
        Create a generative AI model client.
        """
        generative_ai_client = AzureOpenAI(
            azure_endpoint=endpoint,
            azure_ad_token_provider=bearer_token_provider,
            api_version=api_version,
        )
        return generative_ai_client

    @staticmethod
    def create_async_generative_ai_client(endpoint: str, bearer_token_provider, api_version: str):
        """
        Create a generative AI model client in an async call.
        """
        generative_ai_client = AsyncAzureOpenAI(
            azure_endpoint=endpoint,
            azure_ad_token_provider=bearer_token_provider,
            api_version=api_version,
        )
        return generative_ai_client

    @staticmethod
    def get_system_prompt(prompt: str):
        """
        Get a prompt.
        """
        return OpenaiAzureFoundryUtility.get_prompt(prompt, 'system')

    @staticmethod
    def get_prompt(prompt: str, prompt_role: str = 'user'):
        """
        Get a prompt.
        """
        chat_prompt = [
            {
                'role': f'{prompt_role}',
                'content': [
                    {
                        'type': 'text',
                        'text': prompt
                    }
                ]
            }
        ]
        return chat_prompt

    @staticmethod
    def request_generative_ai_completion_using_multiple_prompts(
        generative_ai_client,
        generative_model_configuration,
        generative_model_parameters,
        prompts):
        """
        Get a generative AI completion using multiple prompts.
        """
        generative_ai_completion = generative_ai_client.chat.completions.create(
            model=\
                generative_model_configuration['deployment'],
            messages=\
                prompts,
            max_tokens=\
                generative_model_parameters['azure_openai_client_parameter_max_tokens'],
            temperature=\
                generative_model_parameters['azure_openai_client_parameter_temperature'],
            top_p=\
                generative_model_parameters['azure_openai_client_parameter_top_p'],
            frequency_penalty=\
                generative_model_parameters['azure_openai_client_parameter_frequency_penalty'],
            presence_penalty=\
                generative_model_parameters['azure_openai_client_parameter_presence_penalty'],
            stop=None,
            stream=False
        )
        return generative_ai_completion

    @staticmethod
    def request_generative_ai_completion_using_single_prompt(
        generative_ai_client,
        generative_model_configuration,
        generative_model_parameters,
        prompt: str):
        """
        Get a generative AI completion using a single prompt.
        """
        messages = OpenaiAzureFoundryUtility.get_prompt(prompt)
        try:
            generative_ai_completion = generative_ai_client.chat.completions.create(
                model=\
                    generative_model_configuration['deployment'],
                messages=\
                    messages,
                max_tokens=\
                    generative_model_parameters['azure_openai_client_parameter_max_tokens'],
                temperature=\
                    generative_model_parameters['azure_openai_client_parameter_temperature'],
                top_p=\
                    generative_model_parameters['azure_openai_client_parameter_top_p'],
                frequency_penalty=\
                    generative_model_parameters['azure_openai_client_parameter_frequency_penalty'],
                presence_penalty=\
                    generative_model_parameters['azure_openai_client_parameter_presence_penalty'],
                stop=None,
                stream=False
            )
            return generative_ai_completion
        except Exception as e:
            DebuggingUtility.error(f"An error occurred while connecting to LLM: {e}")
            raise


    @staticmethod
    def get_chat_response(generative_ai_completion) -> str:
        """
        Retrieve a response content from a completion.
        """
        response_in_json = json.loads(generative_ai_completion.to_json())
        response_choices = response_in_json['choices']
        response_choice = response_choices[0]
        response_message = response_choice['message']
        response_message_content = response_message['content']
        return response_message_content

async def main_test_happy_path():
    """
    An end-to-end test program on a happy path.
    """
    generative_model_configuration = \
        OpenaiAzureFoundryUtility.get_generative_model_configuration()
    print(generative_model_configuration)
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
    prompt = 'Who are you?'
    generative_ai_completion = \
        OpenaiAzureFoundryUtility.request_generative_ai_completion_using_single_prompt(
            generative_ai_client,
            generative_model_configuration,
            generative_model_parameters,
            prompt)
    response_payload = OpenaiAzureFoundryUtility.get_chat_response(generative_ai_completion)
    print('---- DEBUGGING-response_payload ----', response_payload)

if __name__ == "__main__":
    asyncio.run(main_test_happy_path())
