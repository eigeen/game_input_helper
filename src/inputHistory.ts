export class InputHistory {
  private readonly submittedContents: string[] = [];
  private browsingIndex: number | null = null;
  private pendingContent = "";

  recordSubmittedContent(content: string) {
    if (content.length > 0) {
      this.submittedContents.push(content);
    }

    this.stopBrowsingHistory();
  }

  showOlderContent(currentContent: string) {
    if (this.submittedContents.length === 0) {
      return currentContent;
    }

    if (this.browsingIndex === null) {
      this.pendingContent = currentContent;
      this.browsingIndex = this.submittedContents.length - 1;
      return this.submittedContents[this.browsingIndex];
    }

    this.browsingIndex = Math.max(0, this.browsingIndex - 1);
    return this.submittedContents[this.browsingIndex];
  }

  showNewerContent(currentContent: string) {
    if (this.browsingIndex === null) {
      return currentContent;
    }

    if (this.browsingIndex === this.submittedContents.length - 1) {
      const restoredContent = this.pendingContent;
      this.stopBrowsingHistory();
      return restoredContent;
    }

    this.browsingIndex += 1;
    return this.submittedContents[this.browsingIndex];
  }

  stopBrowsingHistory() {
    this.browsingIndex = null;
    this.pendingContent = "";
  }
}
