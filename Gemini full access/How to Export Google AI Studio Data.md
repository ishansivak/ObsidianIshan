# How to Export Google AI Studio Data

Direct, granular usage data for Google AI Studio is not explicitly segmented in a simple "Usage" dashboard. Because AI Studio operates on the underlying Gemini API infrastructure, your usage logs are managed through the **Google Cloud Console**.

### Recommended Method: Google Cloud Console (For API Usage)

If you are using AI Studio with an API key, your usage is tracked under your Google Cloud project:

1.  Navigate to the **[Google Cloud Console](https://console.cloud.google.com/)**.
2.  Ensure you have selected the project associated with your API key.
3.  Go to **APIs & Services** > **Dashboard**.
4.  Locate the **Gemini API** (or Generative Language API) in the list and click it.
5.  View the **Metrics** to see request counts, latency, and error rates.
6.  For raw data, you can use **Logs Explorer** (under **Logging** in the sidebar) to filter and export requests to BigQuery or a CSV.

### Alternative: Google Takeout (For Account Data)

If you are looking for your history of prompts and outputs (rather than technical API metrics):

1.  Go to **[Google Takeout](https://takeout.google.com/)**.
2.  Click **"Deselect all"**.
3.  Find and select **"Google AI Studio"** (if available) or **"Gemini"**.
4.  Proceed to export to receive a data archive via email.

---
*Note: If you export the technical logs from Google Cloud, provide the JSON or CSV files, and I can perform a detailed breakdown of your token usage patterns, cost, and request frequency.*

---
**Related**
- [[Documentation/Introduction]]
- [[Research/Research Index]]
