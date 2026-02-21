/* eslint-disable @masknet/jsx-no-logical */
import type { Message, Tool, UserMessage } from '@xsai/shared-chat'

import { env, exit } from 'node:process'

import uuid from '@insel-null/uuid'
import TextInput from 'ink-text-input'

import { generateText } from '@xsai/generate-text'
import { Box, Text } from 'ink'
import { addMessage, recentMemory, retrieveMemory } from 'plastmem'
import { client } from 'plastmem/client'
import { useCallback, useEffect, useMemo, useRef, useState } from 'react'

import promptTemplate from './docs/PROMPT.md?raw'

import { Header } from './components/header'
import { MessageBox } from './components/message'
import { useConversationId } from './hooks/use-dotenv-storage'
import { useTerminalTitle } from './hooks/use-terminal-title'

client.setConfig({ baseUrl: env.PLASTMEM_BASE_URL ?? 'http://localhost:3000' })

const COMMANDS = [
  { cmd: '/model', desc: 'choose what model to use' },
  { cmd: '/clear', desc: 'clear working memory' },
  { cmd: '/reset', desc: 'reset Haru (dangeriously)' },
  { cmd: '/exit', desc: 'exit Haru' },
]

const buildSystemPrompt = (recentMemoryText: string, sessionStart: Date): string => {
  const now = new Date()
  const elapsedMs = now.getTime() - sessionStart.getTime()
  const elapsedMin = Math.floor(elapsedMs / 60000)
  const elapsed = elapsedMin < 1 ? 'just now' : `${elapsedMin}m ago`
  return (promptTemplate)
    .replace('{recentMemory()}', recentMemoryText)
    .replace('{time}', now.toLocaleString())
    .replace('{session_start_time}', sessionStart.toLocaleString())
    .replace('{elapsed_time}', elapsed)
}

export const ChatApp = () => {
  useTerminalTitle('🌷 Haru')

  const [conversationId, setConversationId] = useConversationId()

  const [input, setInput] = useState('')
  const isCommand = useMemo(() => input.startsWith('/'), [input])
  const filteredCommands = useMemo(() => isCommand
    ? COMMANDS.filter(c => c.cmd.startsWith(input.toLowerCase()))
    : [], [input, isCommand])

  const [messages, setMessages] = useState<Message[]>([])
  const [isLoading, setIsLoading] = useState(false)

  const systemPromptRef = useRef<string>('')
  const sessionStartRef = useRef<Date>(new Date())

  useEffect(() => {
    recentMemory({ body: { conversation_id: conversationId } })
      .then(({ data }) => {
        systemPromptRef.current = buildSystemPrompt(data ?? '', sessionStartRef.current)
      })
      .catch(() => {
        systemPromptRef.current = buildSystemPrompt('', sessionStartRef.current)
      })
  }, [conversationId])

  const handleSubmit = useCallback(async (value: string) => {
    if (!value.trim())
      return

    setInput('')

    if (value.startsWith('/')) {
      if (value === '/clear') {
        setMessages([])
      }
      else if (value === '/exit') {
        exit(0)
      }
      else if (value === '/reset') {
        const newId = uuid.v7()
        setConversationId(newId)
        setMessages([])
      }
      return
    }

    const userMsg: UserMessage = { content: value, role: 'user' }
    setMessages(prev => [...prev, userMsg])

    addMessage({ body: { conversation_id: conversationId, message: { content: value, role: 'user' } } }).catch(() => {})

    setIsLoading(true)

    const retrieveTool: Tool = {
      execute: async (input) => {
        const { query } = input as { query: string }
        const { data } = await retrieveMemory({ body: { conversation_id: conversationId, query } }) as { data?: string }
        return data ?? ''
      },
      function: {
        description: 'Search long-term memory for relevant facts and past episodes',
        name: 'retrieve_memory',
        parameters: {
          properties: { query: { description: 'Search query', type: 'string' } },
          required: ['query'],
          type: 'object',
        },
      },
      type: 'function',
    }

    try {
      const history = messages.filter(m => m.role === 'user' || m.role === 'assistant')
      const result = await generateText({
        apiKey: env.OPENAI_API_KEY,
        baseURL: env.OPENAI_BASE_URL!,
        maxSteps: 5,
        messages: [
          { content: systemPromptRef.current, role: 'system' },
          ...history,
          userMsg,
        ],
        model: env.OPENAI_CHAT_MODEL!,
        tools: [retrieveTool],
      })

      const text = result.text ?? ''
      setMessages(prev => [...prev, { content: text, role: 'assistant' }])
      addMessage({ body: { conversation_id: conversationId, message: { content: text, role: 'assistant' } } }).catch(() => {})
    }
    catch (err) {
      setMessages(prev => [...prev, { content: `error: ${String(err)}`, role: 'assistant' }])
    }
    finally {
      setIsLoading(false)
    }
  }, [messages, conversationId, setConversationId])

  return (
    <Box flexDirection="column">
      <Header />
      {messages.map((message, index) => (
        // eslint-disable-next-line react/no-array-index-key
        <MessageBox key={`message ${index}`} message={message} />
      ))}

      {isLoading && (
        <Box padding={1}>
          <Text dimColor>...</Text>
        </Box>
      )}

      <Box backgroundColor="grey" padding={1}>
        <Box marginRight={1}>
          <Text>›</Text>
        </Box>
        <TextInput
          data-test-id="text-input"
          onChange={setInput}
          onSubmit={value => void handleSubmit(value)}
          placeholder="Write a message..."
          showCursor
          value={input}
        />
      </Box>

      <Box paddingX={1} paddingY={1}>
        {isCommand
          ? (
              <Box flexDirection="column">
                {filteredCommands.length > 0
                  ? (
                      filteredCommands.map((item, i) => (
                        <Box gap={2} key={i}>
                          <Text bold={i === 0} color={i === 0 ? 'blue' : undefined}>{item.cmd}</Text>
                          <Text bold={i === 0} color={i === 0 ? 'blue' : undefined} dimColor={i !== 0}>{item.desc}</Text>
                        </Box>
                      ))
                    )
                  : (
                      <Text dimColor>no matches</Text>
                    )}
              </Box>
            )
          : <Text dimColor>/ for commands</Text>}
      </Box>
    </Box>
  )
}
