"use client";
import { useState } from "react";
export default function Home() {
  const [file, setFile] = useState<File | null>(null);
  const [transcript, setTranscript] = useState<any>(null);
  const [loading, setLoading] = useState(false);
  const handleUpload = async () => {
    if (!file) return;
    setLoading(true);
    const form = new FormData();
    form.append("audio", file);
    const res = await fetch("/api/transcribe", { method: "POST", body: form });
    const data = await res.json();
    setTranscript(data);
    setLoading(false);
  };
  return (
    <main className="min-h-screen bg-gradient-to-br from-blue-900 via-black to-cyan-900 text-white p-8">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-5xl font-bold mb-4 bg-gradient-to-r from-blue-400 to-cyan-400 bg-clip-text text-transparent">otter</h1>
        <p className="text-xl text-gray-300 mb-8">Auto-transcribe your meetings</p>
        <div className="bg-white/10 backdrop-blur-lg rounded-2xl p-8 mb-8">
          <input type="file" accept="audio/*" onChange={(e) => setFile(e.target.files?.[0] || null)}
            className="block w-full text-sm text-gray-300 file:mr-4 file:py-3 file:px-6 file:rounded-full file:border-0 file:bg-blue-600 file:text-white hover:file:bg-blue-500 cursor-pointer" />
          <button onClick={handleUpload} disabled={!file || loading}
            className="mt-4 px-8 py-3 bg-gradient-to-r from-blue-600 to-cyan-600 rounded-full font-semibold hover:opacity-90 disabled:opacity-50 transition">
            {loading ? "Transcribing..." : "Transcribe Meeting"}
          </button>
        </div>
        {transcript && (
          <div className="bg-white/10 backdrop-blur rounded-2xl p-6">
            <h2 className="text-2xl font-semibold mb-2">{transcript.title}</h2>
            <p className="text-gray-400 mb-4">Duration: {transcript.duration}</p>
            <div className="space-y-3">
              {transcript.segments?.map((seg: any, i: number) => (
                <div key={i} className="flex gap-4">
                  <span className="text-blue-400 font-mono text-sm whitespace-nowrap">{seg.start.toFixed(1)}s</span>
                  <div><span className="text-cyan-400 font-semibold">{seg.speaker}: </span><span>{seg.text}</span></div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </main>
  );
}