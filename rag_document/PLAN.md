# RAG Document Query Tool - Implementation Plan

## Project Overview

Build a local RAG (Retrieval-Augmented Generation) pipeline to query PDF documents with context-aware answers and source citations.

## Architecture

```
PDFs → Parser → Chunker → Embeddings → Vector DB (ChromaDB)
                                            ↓
                                    Query Interface (MCP/CLI)
                                            ↓
                                Retriever → LLM (Claude API) → Answer with Citations
```

## Requirements Summary

- **Scale**: Small collection (< 100 PDF documents)
- **Privacy**: Hybrid approach (local embeddings, cloud LLM)
- **Interface**: MCP server (for Claude Code), CLI, API server
- **Features**: Multi-document chat, source citations with page numbers, document preprocessing (OCR, tables)

## Technology Stack

### Document Processing
- **PyMuPDF (fitz)** or **pdfplumber**: PDF parsing with page number preservation
- **tesseract**: OCR for scanned PDFs (if needed)
- **unstructured** (optional): For handling diverse document types

### Embeddings (Local)
- **sentence-transformers**: Local embedding generation
  - Model: `all-MiniLM-L6-v2` (lightweight, fast) or `bge-small-en-v1.5` (better quality)
  - No API costs, runs locally

### Vector Database
- **ChromaDB**: Primary choice
  - Zero-config, persistent storage
  - Good metadata support for citations
  - Perfect for < 100 docs
- **FAISS** (alternative): Faster but less metadata support

### RAG Framework
- **LlamaIndex**: Primary choice
  - Built-in citation support
  - Simpler than LangChain for this use case
  - Better metadata tracking
- **LangChain** (alternative): More flexibility, more complex

### LLM
- **Claude API** (Anthropic): Via anthropic SDK
- **Hybrid approach**: Local embeddings, cloud generation

### Interface
- **MCP Server**: FastAPI-based MCP server for Claude Code integration
- **CLI**: typer or argparse-based command-line tool
- **API**: FastAPI REST endpoints (optional)

## Project Structure

```
rag-doc-query/
├── README.md
├── requirements.txt
├── config.yaml                # Configuration (paths, model settings)
├── .env                       # API keys (ANTHROPIC_API_KEY)
│
├── documents/                 # Input PDF storage
│   └── .gitkeep
│
├── data/                      # Persistent data
│   └── chroma/                # ChromaDB vector store
│       └── .gitkeep
│
├── src/
│   ├── __init__.py
│   ├── parser.py              # PDF parsing & chunking logic
│   ├── indexer.py             # Embedding generation & vector DB storage
│   ├── retriever.py           # Query & retrieval logic
│   ├── generator.py           # LLM integration & response generation
│   ├── mcp_server.py          # MCP server for Claude Code
│   └── utils.py               # Shared utilities
│
├── cli.py                     # CLI interface entry point
└── tests/                     # Unit tests
    └── __init__.py
```

## Implementation Steps

### Phase 1: Setup & Foundation
1. Create project structure and directories
2. Set up Python virtual environment (`python -m venv venv`)
3. Create `requirements.txt` with core dependencies:
   ```
   llama-index
   llama-index-embeddings-huggingface
   llama-index-llms-anthropic
   chromadb
   sentence-transformers
   PyMuPDF
   pdfplumber
   anthropic
   fastapi
   uvicorn
   typer
   python-dotenv
   pyyaml
   ```
4. Create `config.yaml` for configuration management
5. Set up `.env` for API keys

### Phase 2: Document Parser (`src/parser.py`)
**Purpose**: Extract text from PDFs while preserving structure and metadata

**Key Functions**:
- `parse_pdf(file_path) -> List[Document]`
  - Extract text with page numbers
  - Handle text-based and scanned PDFs (OCR if needed)
  - Preserve document structure (headings, paragraphs)

- `chunk_document(document, chunk_size=512, overlap=50) -> List[Chunk]`
  - Split document into semantic chunks
  - Maintain page number metadata for each chunk
  - Overlap for context continuity

**Metadata to Preserve**:
- Filename
- Page number(s) for each chunk
- Document title (if extractable)
- Date/author (if available)

### Phase 3: Indexer (`src/indexer.py`)
**Purpose**: Generate embeddings and store in vector database

**Key Functions**:
- `initialize_embedding_model() -> EmbeddingModel`
  - Load sentence-transformers model locally
  - Cache model for reuse

- `index_documents(document_dir) -> VectorStore`
  - Parse all PDFs in directory
  - Generate embeddings for each chunk
  - Store in ChromaDB with metadata
  - Return vector store handle

- `add_document(file_path) -> bool`
  - Incremental indexing for new documents
  - Update vector store

- `list_indexed_documents() -> List[str]`
  - Query ChromaDB for indexed document list

**ChromaDB Setup**:
- Persistent storage in `data/chroma/`
- Collection name: "documents"
- Metadata fields: filename, page_number, chunk_id, title

### Phase 4: Retriever (`src/retriever.py`)
**Purpose**: Query vector database and retrieve relevant chunks

**Key Functions**:
- `retrieve_context(query, top_k=5) -> List[Chunk]`
  - Generate query embedding
  - Search ChromaDB for similar chunks
  - Return top_k results with metadata

- `format_context_with_citations(chunks) -> str`
  - Format retrieved chunks for LLM prompt
  - Include source citations (filename, page number)
  - Structure for clear attribution

**Retrieval Strategy**:
- Semantic similarity search
- Optional: Re-ranking with cross-encoder (if needed)
- Include metadata in results

### Phase 5: Generator (`src/generator.py`)
**Purpose**: Generate answers using Claude API with retrieved context

**Key Functions**:
- `generate_answer(query, context_chunks) -> Answer`
  - Format prompt with context and citations
  - Call Claude API (claude-3-5-sonnet)
  - Parse response

- `format_response(answer, sources) -> str`
  - Format final answer with source citations
  - Include page references
  - Pretty print for CLI/MCP

**Prompt Engineering**:
- System prompt: Emphasize citation accuracy
- Context format: Clear source attribution
- Instruction: "Always cite sources with [filename, page X]"

### Phase 6: MCP Server (`src/mcp_server.py`)
**Purpose**: Expose RAG system to Claude Code via MCP protocol

**MCP Tools to Expose**:
1. `query_documents`
   - Input: query string
   - Output: answer with citations

2. `index_document`
   - Input: file path
   - Output: success/failure status

3. `list_documents`
   - Input: none
   - Output: list of indexed documents

**Implementation**:
- FastAPI-based MCP server
- Follow MCP protocol specification
- Register with Claude Code

**MCP Server Configuration**:
- Default port: 8000
- Endpoint: `/mcp`
- Health check: `/health`

### Phase 7: CLI Interface (`cli.py`)
**Purpose**: Command-line tool for standalone usage

**Commands**:
- `index <directory>`: Index all PDFs in directory
- `add <file>`: Add single document to index
- `query "<question>"`: Query indexed documents
- `list`: List all indexed documents
- `clear`: Clear vector database

**Implementation**:
- Use `typer` for clean CLI interface
- Progress bars for indexing
- Formatted output with colors
- Configuration loading from `config.yaml`

### Phase 8: Configuration & Documentation
**config.yaml**:
```yaml
documents:
  directory: "./documents"

vector_db:
  type: "chromadb"
  persist_directory: "./data/chroma"
  collection_name: "documents"

embeddings:
  model: "all-MiniLM-L6-v2"
  device: "cpu"  # or "cuda" if GPU available

llm:
  provider: "anthropic"
  model: "claude-3-5-sonnet-20241022"
  temperature: 0.1

chunking:
  chunk_size: 512
  overlap: 50

retrieval:
  top_k: 5

mcp:
  host: "0.0.0.0"
  port: 8000
```

**README.md**:
- Installation instructions
- Quick start guide
- Usage examples
- MCP server setup for Claude Code
- Configuration options

## Key Implementation Details

### Citation Strategy
1. **Chunk Metadata**: Store page numbers with each chunk
2. **Retrieval**: Include metadata in retrieved results
3. **Prompt Engineering**: Instruct Claude to cite sources
4. **Format**: Use `[filename.pdf, page X]` format

### Chunking Strategy
- **Size**: 512 tokens (~400 words) per chunk
- **Overlap**: 50 tokens to preserve context
- **Boundaries**: Try to split on paragraph/sentence boundaries
- **Metadata**: Preserve page numbers, section headers

### Performance Optimization
- **Caching**: Cache embedding model in memory
- **Batch Processing**: Process multiple PDFs in parallel
- **Incremental Indexing**: Only re-index changed documents
- **Query Caching**: Cache recent queries (optional)

## Testing Strategy

### Unit Tests
- Test PDF parsing with sample documents
- Test chunking logic
- Test embedding generation
- Test retrieval accuracy

### Integration Tests
- End-to-end query flow
- MCP server functionality
- CLI commands

### Test Documents
- Create `tests/fixtures/` with sample PDFs
- Include text-based and scanned PDFs
- Various formats and structures

## Deployment & Usage

### Installation
```bash
git clone <repo>
cd rag-doc-query
python -m venv venv
source venv/bin/activate  # or `venv\Scripts\activate` on Windows
pip install -r requirements.txt
cp .env.example .env
# Add ANTHROPIC_API_KEY to .env
```

### Initial Setup
```bash
# Index documents
python cli.py index documents/

# Query
python cli.py query "What are the main findings?"
```

### MCP Server for Claude Code
```bash
# Start MCP server
python -m src.mcp_server

# In Claude Code settings, add MCP server:
# http://localhost:8000/mcp
```

## Future Enhancements (Optional)

1. **Multi-format Support**: Word docs, text files, HTML
2. **Advanced Chunking**: Semantic chunking with LLM
3. **Re-ranking**: Cross-encoder for better relevance
4. **Conversation Memory**: Multi-turn conversations with context
5. **Document Summaries**: Pre-generate summaries for quick overview
6. **Web UI**: Streamlit/Gradio interface
7. **Batch Queries**: Process multiple questions at once
8. **Export Results**: Save Q&A sessions to file

## Estimated Timeline

- **Phase 1-2** (Setup & Parser): 2-3 hours
- **Phase 3-4** (Indexer & Retriever): 3-4 hours
- **Phase 5** (Generator): 2 hours
- **Phase 6** (MCP Server): 2-3 hours
- **Phase 7** (CLI): 1-2 hours
- **Phase 8** (Docs & Config): 1 hour
- **Testing & Polish**: 2-3 hours

**Total**: ~15-20 hours for complete implementation

## Success Criteria

- [ ] Can index 100 PDFs in < 5 minutes
- [ ] Retrieval returns relevant chunks with correct page numbers
- [ ] Claude generates accurate answers with proper citations
- [ ] MCP server integrates seamlessly with Claude Code
- [ ] CLI is intuitive and functional
- [ ] All tests pass
- [ ] Documentation is complete

## Resources & References

- **LlamaIndex Docs**: https://docs.llamaindex.ai/
- **ChromaDB Docs**: https://docs.trychroma.com/
- **MCP Protocol**: https://spec.modelcontextprotocol.io/
- **sentence-transformers**: https://www.sbert.net/
- **Anthropic API**: https://docs.anthropic.com/

## Notes

- Start simple, iterate based on results
- Test with real documents early
- Focus on citation accuracy - it's the key feature
- MCP integration makes this much more powerful with Claude Code
- Consider using LlamaIndex's built-in tools rather than reinventing
