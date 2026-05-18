import { useState, useRef, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Send, X, Bot, User } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useChatStore } from '../stores/chatStore';
import '../styles/ChatWindow.css';

interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: Date;
}

const ChatWindow = () => {
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const { messages, addMessage, clearMessages } = useChatStore();
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSend = async () => {
    if (!input.trim() || isLoading) return;

    const userMessage: Message = {
      id: Date.now().toString(),
      role: 'user',
      content: input.trim(),
      timestamp: new Date(),
    };

    addMessage(userMessage);
    setInput('');
    setIsLoading(true);

    try {
      // Call Rust backend to get AI response
      const response = await invoke<Message>('send_message', {
        message: userMessage.content,
        modelConfig: {
          provider: 'ollama',
          modelName: 'qwen2.5:1.5b',
          baseUrl: 'http://localhost:11434',
        },
      });

      addMessage(response);
    } catch (error) {
      console.error('Failed to send message:', error);
      addMessage({
        id: Date.now().toString(),
        role: 'assistant',
        content: 'Sorry, I encountered an error. Please check your connection or model settings.',
        timestamp: new Date(),
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  const handleClose = async () => {
    const window = getCurrentWindow();
    await window.hide();
  };

  return (
    <div className="chat-window">
      <div className="chat-header">
        <div className="chat-title">
          <Bot size={20} />
          <span>Chat with Terminal Cat</span>
        </div>
        <button className="close-button" onClick={handleClose}>
          <X size={18} />
        </button>
      </div>

      <div className="chat-messages">
        <AnimatePresence>
          {messages.length === 0 && (
            <motion.div
              className="welcome-message"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
            >
              <Bot size={48} />
              <p>Hello! I'm Terminal Cat 🐱</p>
              <p className="welcome-subtitle">
                How can I help you today?
              </p>
            </motion.div>
          )}

          {messages.map((message, index) => (
            <motion.div
              key={message.id}
              className={`message ${message.role}`}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: index * 0.05 }}
            >
              <div className="message-avatar">
                {message.role === 'user' ? <User size={16} /> : <Bot size={16} />}
              </div>
              <div className="message-content">
                <p>{message.content}</p>
                <span className="message-time">
                  {message.timestamp.toLocaleTimeString([], {
                    hour: '2-digit',
                    minute: '2-digit',
                  })}
                </span>
              </div>
            </motion.div>
          ))}

          {isLoading && (
            <motion.div
              className="message assistant loading"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
            >
              <div className="message-avatar">
                <Bot size={16} />
              </div>
              <div className="message-content">
                <div className="typing-indicator">
                  <span></span>
                  <span></span>
                  <span></span>
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
        <div ref={messagesEndRef} />
      </div>

      <div className="chat-input-container">
        <div className="chat-input-wrapper">
          <textarea
            className="chat-input"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyPress={handleKeyPress}
            placeholder="Type a message..."
            rows={1}
          />
          <button
            className="send-button"
            onClick={handleSend}
            disabled={!input.trim() || isLoading}
          >
            <Send size={18} />
          </button>
        </div>
      </div>
    </div>
  );
};

export default ChatWindow;
