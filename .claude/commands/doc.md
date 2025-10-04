---
description: Add comprehensive documentation to the currently open file
argument-hint: [file_path]
allowed-tools: Read, Edit
---

Please add or update comprehensive documentation to the currently open file or the files that have been modified following Python best practices and PEP 257 conventions.

Arguments provided: $ARGUMENTS

If a file path is provided as an argument, use that file. Otherwise, use all files that have changed.

For Python files, add:

1. **Module docstring** at the top of the file explaining:
   - Purpose of the module
   - Key components or classes
   - Usage examples if applicable

2. **Class docstrings** with:
   - Purpose and responsibility of the class
   - Attributes section listing all instance/class attributes with types and descriptions
   - Usage examples for complex classes

3. **Function/method docstrings** with:
   - Clear description of what the function does
   - Args section with parameter names, types, and descriptions
   - Returns section with return type and description
   - Raises section for any exceptions thrown

4. **Inline comments** for complex logic that isn't self-explanatory

Follow these guidelines:

- Use triple double-quotes for docstrings (`"""`)
- First line should be a one-line summary
- Add a blank line before detailed description
- Use Google or NumPy docstring style for consistency
- Keep descriptions concise but informative
- Don't document obvious code - focus on "why" not "what"
- Use proper type hints in function signatures when possible

Example format:

```python
"""Module description here.

More detailed explanation of the module's purpose and contents.
"""

class Example:
    """Brief description of the class.

    Detailed explanation of the class purpose and behavior.

    Attributes:
        attr_name: Description of the attribute
        another_attr: Description of another attribute
    """

    def method(self, param: str) -> bool:
        """Brief description of what this method does.

        More detailed explanation if needed.

        Args:
            param: Description of the parameter

        Returns:
            Description of the return value

        Raises:
            ValueError: When param is invalid
        """
```

After reading the file, analyze its purpose and add appropriate documentation without changing the actual code logic.
