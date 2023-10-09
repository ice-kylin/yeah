export interface Message {
    msg_type: "Primary" | "Success" | "Warning" | "Danger";
    title: string;
    content: string;
}
