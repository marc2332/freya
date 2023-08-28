import SyntaxHighlighter from 'react-syntax-highlighter';
import { atomOneDark } from 'react-syntax-highlighter/dist/cjs/styles/hljs/index';

export function CodeBlock({ code }) {
	return (
		<SyntaxHighlighter  language="rust" style={atomOneDark}>
			{code}
		</SyntaxHighlighter>
	)
}